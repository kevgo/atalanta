use crate::probes::Stack;
use crate::Outcome;
use std::io::Write;
use std::str;
use tabwriter::TabWriter;

/// lists all available commands
pub fn list(stack: impl Stack) -> Outcome {
    println!("Available commands ({}):", stack);
    let mut tw = TabWriter::new(vec![]);
    for task in stack.tasks() {
        let text = format!("{}\t{}", task.cmd, task.desc.as_deref().unwrap_or_default());
        tw.write(text.as_bytes()).unwrap();
    }
    let bytes = tw.into_inner().unwrap();
    unsafe {
        println!("{}", str::from_utf8_unchecked(&bytes));
    }
    Outcome::Ok
}
