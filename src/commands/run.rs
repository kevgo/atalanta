use crate::cli;
use crate::domain::{Outcome, Stacks, Task, Tasks};
use std::process::Stdio;

pub fn run(stacks: Stacks, name: String) -> Outcome {
  let task_names = stacks.tasks_matching_name(&name);
  let task_name = match task_names.len() {
    0 => {
      return Outcome::UnknownTask { task: name, stacks };
    }
    1 => task_names[0],
    _ if task_names.contains(&name.as_ref()) => &name,
    _ => {
      let tasks: Vec<Task> = task_names
        .into_iter()
        .map(|task_name| stacks.task_with_name(task_name).unwrap().clone())
        .collect();
      return Outcome::TooManyTaskMatches {
        tasks: Tasks::from(tasks),
      };
    }
  };
  let task = stacks.task_with_name(task_name).unwrap();
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
