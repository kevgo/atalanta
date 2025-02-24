use crate::cli;
use crate::domain::{Outcome, Stacks, Tasks};
use std::process::Stdio;

pub fn run(stacks: Stacks, name: String) -> Outcome {
  let tasks = stacks.tasks_matching_name(&name);
  let task = match tasks.len() {
    0 => {
      return Outcome::UnknownTask { task: name, stacks };
    }
    1 => tasks[0],
    _ => {
      return Outcome::TooManyTaskMatches {
        tasks: Tasks::from(tasks),
      };
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
