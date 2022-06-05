use crate::domain::{Outcome, Workspace};
use std::process::{Command, Stdio};

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
    let output = Command::new(&task.cmd)
        .args(&task.argv)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();
    match output {
        Ok(output) => match output.status.code().unwrap() {
            0 => Outcome::Success,
            exit_code => Outcome::ScriptFailed {
                exit_code: exit_code as u8,
            },
        },
        Err(e) => Outcome::CannotRunExecutable { err: e.to_string() },
    }
}
