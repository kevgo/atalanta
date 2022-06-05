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
        Ok(output) => Outcome::Success {
            exit_code: output.status.code().unwrap() as u8,
        },
        Err(e) => Outcome::CannotRunExecutable { err: e.to_string() },
    }
}
