use crate::cli;
use crate::domain::{Outcome, Stacks, Task, Tasks};
use std::process::Stdio;

pub fn run(stacks: Stacks, name: String) -> Outcome {
  let tasks = stacks.tasks_matching_name(&name);
  let task = match tasks.len() {
    0 => {
      return Outcome::UnknownTask { task: name, stacks };
    }
    1 => tasks[0],
    _ => {
      // check for direct match
      if let Some(task) = find_direct_match(&tasks, &name) {
        task
      } else {
        return Outcome::TooManyTaskMatches {
          tasks: Tasks::from(tasks),
        };
      }
    }
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

fn find_direct_match<'a, 'b>(tasks: &'a Vec<&'b Task>, name: &str) -> Option<&'a Task> {
  for task in tasks {
    if task.name == name {
      return Some(task);
    }
  }
  None
}
