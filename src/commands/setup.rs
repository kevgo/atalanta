use super::run::reduce_exit_status_to_code;
use crate::domain::{Outcome, Workspace};
use std::process::Stdio;

pub fn setup(workspace: Workspace) -> Outcome {
  let mut executed = false;
  for stack in workspace.stacks {
    let Some(mut cmd) = stack.setup() else {
      continue;
    };
    executed = true;
    let output = cmd
      .stdin(Stdio::inherit())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .output();
    let output = match output {
      Ok(output) => output,
      Err(e) => return Outcome::CannotFindExecutable { err: e.to_string() },
    };
    if let Some(exit_code) = output.status.code() {
      if exit_code != 0 {
        return Outcome::ScriptFailed {
          exit_code: reduce_exit_status_to_code(exit_code),
        };
      }
    };
  }
  if executed {
    Outcome::Success
  } else {
    Outcome::NoSetup
  }
}
