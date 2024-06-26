use crate::domain::{Outcome, Task, Workspace};
use std::process::Stdio;

pub fn run(workspace: Workspace, name: String) -> Outcome {
  let task_names = workspace.tasks_matching_name(&name);
  let task_name = match task_names.len() {
    0 => {
      return Outcome::UnknownTask {
        task: name,
        workspace,
      }
    }
    1 => task_names[0],
    _ if task_names.contains(&name.as_ref()) => &name,
    _ => {
      let tasks: Vec<Task> = task_names
        .iter()
        .map(|task_name| workspace.task_with_name(task_name).unwrap().clone())
        .collect();
      return Outcome::TooManyTaskMatches { tasks };
    }
  };
  let task = workspace.task_with_name(task_name).unwrap();
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
      exit_code: reduce_exit_status_to_code(exit_code),
    },
    None => Outcome::ScriptFailed { exit_code: 255 },
  }
}

pub fn reduce_exit_status_to_code(code: i32) -> u8 {
  if !(0..=255).contains(&code) {
    return 255;
  }
  u8::try_from(code).unwrap()
}
