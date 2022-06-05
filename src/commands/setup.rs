use crate::domain::{Outcome, Workspace};
use std::process::{Command, Stdio};

pub fn setup(workspace: Workspace) -> Outcome {
    let mut executed = false;
    for stack in workspace.stacks {
        let (cmd, args) = match stack.setup() {
            Some(setup) => setup,
            None => continue,
        };
        let output = Command::new(cmd)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();
        match output {
            Ok(output) => {
                executed = true;
                if let Some(exit_code) = output.status.code() {
                    if exit_code != 0 {
                        return Outcome::Success {
                            exit_code: exit_code as u8,
                        };
                    }
                };
            }
            Err(e) => return Outcome::CannotRunExecutable { err: e.to_string() },
        };
    }
    if !executed {
        println!("Warning: I don't know how to set up this workspace");
    }
    Outcome::Success { exit_code: 0 }
}
