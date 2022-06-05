use crate::{Outcome, Stacks};
use std::io::Write;
use std::str;
use tabwriter::TabWriter;

/// lists all available commands
pub fn list(stacks: Stacks) -> Outcome {
    for stack in stacks {
        println!("Commands ({}):\n", stack);
        let mut tw = TabWriter::new(vec![]);
        let tasks = match stack.tasks() {
            Ok(tasks) => tasks,
            Err(outcome) => return outcome,
        };
        for task in tasks {
            let desc = match task.desc {
                Some(desc) => desc,
                None => task.cmd,
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
