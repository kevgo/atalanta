use crate::cli;
use crate::domain::{Outcome, Stacks};
use std::process::Stdio;

pub(crate) fn setup(stacks: Stacks) -> Outcome {
  let mut executed = false;
  for stack in stacks {
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
          exit_code: cli::exit_status_to_code(exit_code),
        };
      }
    }
  }
  if executed {
    Outcome::Success
  } else {
    Outcome::NoSetup
  }
}
