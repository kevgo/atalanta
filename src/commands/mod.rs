use std::fmt::Display;
use std::io::Write;

use tabwriter::TabWriter;

use crate::probes::Stack;
use crate::Outcome;

/// lists all available commands
pub fn list<S: Stack + Display>(stack: S) -> Outcome {
    println!("Available commands ({}):", stack);
    let mut tw = TabWriter::new(vec![]);
    for task in stack.tasks() {
        let text = format!("{}\t{}", task.cmd, task.desc.as_deref().unwrap_or_default());
        tw.write(text.as_bytes()).unwrap();
    }
    println!(
        "{}",
        std::str::from_utf8(tw.into_inner().unwrap().as_ref()).unwrap()
    );
    Outcome::Ok
}
