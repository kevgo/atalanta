use crate::cli;
use crate::domain::{Outcome, Task, Workspace};
use std::process::Stdio;

pub fn run(workspace: Workspace, name: String) -> Outcome {
  let tasks = workspace.tasks_matching_name(&name);
  let task = match tasks.len() {
    0 => {
      return Outcome::UnknownTask {
        task: name,
        workspace,
      };
    }
    1 => tasks[0],
    _ => {
      let exact_match = tasks.find_by_name(name);
      let tasks: Vec<Task> = tasks
        .iter()
        .map(|task_name| workspace.task_with_name(task_name).unwrap().clone())
        .collect();
      return Outcome::TooManyTaskMatches { tasks };
    }
  };
  let task = workspace.task_with_name(task).unwrap();
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
