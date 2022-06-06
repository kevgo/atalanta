use crate::domain::{Outcome, Workspace};
use std::process::{Command, Stdio};

pub fn setup(workspace: Workspace) -> Outcome {
    let mut executed = false;
    for stack in workspace.stacks {
        let (cmd, args) = match stack.setup() {
            Some(setup) => setup,
            None => continue,
        };
        executed = true;
        let output = Command::new(cmd)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();
        match output {
            Ok(output) => {
                if let Some(exit_code) = output.status.code() {
                    if exit_code != 0 {
                        return Outcome::ScriptFailed {
                            exit_code: exit_code as u8,
                        };
                    }
                };
            }
            Err(e) => return Outcome::CannotFindExecutable { err: e.to_string() },
        };
    }
    if executed {
        Outcome::Success
    } else {
        Outcome::NoSetup
    }
}
