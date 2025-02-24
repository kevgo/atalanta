use crate::cli;
use crate::domain::{Outcome, Stacks, Task, Tasks};
use std::process::Stdio;

pub fn run(stacks: Stacks, name: String) -> Outcome {
  let tasks = stacks.tasks_fuzzy_matching_name(&name);
  let task = match tasks.len() {
    0 => {
      return Outcome::UnknownTask { task: name, stacks };
    }
    1 => tasks[0],
    _ => match exact_match(&tasks, &name) {
      Some(task) => task,
      None => {
        return Outcome::TooManyTaskMatches {
          tasks: Tasks::from(tasks),
        };
      }
    },
  };
  let output = task
    .command()
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output();
  let output = match output {
    Ok(output) => output,
    Err(e) => return Outcome::CannotFindExecutable { err: e.to_string() },
  };
  match output.status.code() {
    Some(0) => Outcome::Success,
    Some(exit_code) => Outcome::ScriptFailed {
      exit_code: cli::exit_status_to_code(exit_code),
    },
    None => Outcome::ScriptFailed { exit_code: 255 },
  }
}

fn exact_match<'a>(tasks: &'a Vec<&'_ Task>, name: &str) -> Option<&'a Task> {
  tasks.iter().find(|&&task| task.name == name).copied()
}

#[cfg(test)]
mod tests {

  mod exact_match {
    use crate::domain::Task;
    use big_s::S;

    #[test]
    fn has_match() {
      let task_1 = Task {
        name: S("one"),
        ..Task::default()
      };
      let task_2 = Task {
        name: S("onetwo"),
        ..Task::default()
      };
      let tasks = vec![&task_1, &task_2];
      let have = super::super::exact_match(&tasks, "one");
      let want = Some(&task_1);
      assert_eq!(have, want);
    }

    #[test]
    fn no_match() {
      let task = Task {
        name: S("onetwo"),
        ..Task::default()
      };
      let tasks = vec![&task];
      let have = super::super::exact_match(&tasks, "one");
      let want = None;
      assert_eq!(have, want);
    }
  }
}
