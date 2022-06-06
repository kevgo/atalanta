use crate::domain::{Outcome, Workspace};
use std::io::Write;
use std::str;
use tabwriter::TabWriter;

/// lists all available commands
pub fn list(workspace: Workspace) -> Outcome {
    for stack in workspace.stacks {
        println!("{}:\n", stack);
        let mut tw = TabWriter::new(vec![]);
        for task in stack.tasks() {
            let text = format!("{}\t{}\n", task.name, task.desc);
            tw.write(text.as_bytes()).unwrap();
        }
        let bytes = tw.into_inner().unwrap();
        unsafe {
            println!("{}", str::from_utf8_unchecked(&bytes));
        }
    }
    Outcome::Success
}
