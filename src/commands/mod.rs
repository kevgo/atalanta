use std::io::Write;

use tabwriter::TabWriter;

use crate::probes::{Stack, Task};
use crate::Outcome;

/// lists all available commands
pub fn list(stack: Stack, tasks: &[Task]) -> Outcome {
    println!("Available commands ({}):", stack);
    let mut tw = TabWriter::new(vec![]);
    for task in tasks {
        let text = format!("{}\t{}", task.cmd, task.desc.as_deref().unwrap_or_default());
        tw.write(text.as_bytes()).unwrap();
    }
    tw.flush().unwrap();
    Outcome::Ok
}
