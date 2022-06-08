use crate::domain::{Outcome, Workspace};
use std::process::Stdio;

pub fn run(workspace: Workspace, name: String) -> Outcome {
    let tasks = workspace.tasks_matching_name(&name);
    let task = match tasks.len() {
        0 => {
            return Outcome::UnknownTask {
                task: name,
                workspace,
            }
        }
        1 => workspace.task_with_name(tasks[0]).unwrap(),
        _ => {
            let tasks: Vec<String> = tasks.iter().map(|task| String::from(*task)).collect();
            return Outcome::TooManyTaskMatches { tasks };
        }
    };
    println!("{}", task.cli());
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
