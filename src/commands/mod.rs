use crate::{Outcome, Workspace};
use std::io::Write;
use std::process::{Command, Stdio};
use std::str;
use tabwriter::TabWriter;

/// lists all available commands
pub fn list(workspace: Workspace) -> Outcome {
    for stack in workspace.stacks {
        println!("{}:\n", stack);
        let mut tw = TabWriter::new(vec![]);
        for task in stack.tasks() {
            let desc = match &task.desc {
                Some(desc) => desc,
                None => &task.cmd,
            };
            let text = format!("{}\t{}\n", task.name, desc);
            tw.write(text.as_bytes()).unwrap();
        }
        let bytes = tw.into_inner().unwrap();
        unsafe {
            println!("{}", str::from_utf8_unchecked(&bytes));
        }
    }
    Outcome::Ok
}

pub fn run(workspace: Workspace, name: String) -> Outcome {
    let (stack, task) = match workspace.task_with_name(&name) {
        Some(task) => task,
        None => return Outcome::UnknownTask(name, workspace),
    };
    let argv = match shellwords::split(&task.cmd) {
        Ok(argv) => argv,
        Err(_) => {
            return Outcome::MismatchedQuotesInCmd {
                stack: stack.to_string(),
                task: task.name.clone(),
                cmd: task.cmd.clone(),
            }
        }
    };
    let (cmd, args) = argv.split_at(1);
    Command::new(&cmd[0])
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("cannot find executable");
    Outcome::Ok
}
