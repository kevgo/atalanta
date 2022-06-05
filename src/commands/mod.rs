use crate::{Outcome, Stacks};
use std::io::Write;
use std::str;
use tabwriter::TabWriter;

/// lists all available commands
pub fn list(stacks: Stacks) -> Outcome {
    for stack in stacks {
        println!("Available commands ({}):", stack);
        let mut tw = TabWriter::new(vec![]);
        let tasks = match stack.tasks() {
            Ok(tasks) => tasks,
            Err(outcome) => return outcome,
        };
        for task in tasks {
            let text = format!("{}\t{}", task.cmd, task.desc.as_deref().unwrap_or_default());
            tw.write(text.as_bytes()).unwrap();
        }
        let bytes = tw.into_inner().unwrap();
        unsafe {
            println!("{}", str::from_utf8_unchecked(&bytes));
        }
    }
    Outcome::Ok
}
