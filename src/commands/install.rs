use crate::cli;
use crate::domain::{Outcome, Stacks};

pub(crate) fn install(stacks: Stacks) -> Outcome {
  let mut executed = false;
  for stack in stacks {
    let Some(mut cmd) = stack.install() else {
      continue;
    };
    executed = true;
    let status = match cmd.status() {
      Ok(status) => status,
      Err(e) => return Outcome::CannotFindExecutable { err: e.to_string() },
    };
    if let Some(exit_code) = status.code() {
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
    Outcome::NoInstall
  }
}
