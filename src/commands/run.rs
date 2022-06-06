use crate::domain::{Outcome, Workspace};
use std::process::Stdio;

pub fn run(workspace: Workspace, name: String) -> Outcome {
    let task = match workspace.task_with_name(&name) {
        Some(task) => task,
        None => {
            return Outcome::UnknownTask {
                task: name,
                workspace,
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
            exit_code: exit_code as u8,
        },
        None => Outcome::ScriptFailed { exit_code: 255 },
    }
}
