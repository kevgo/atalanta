use crate::domain::{Outcome, Task, Workspace};
use ansi_term::Style;
use std::io::Write;
use std::str;
use tabwriter::TabWriter;

/// lists all available commands
pub fn list(workspace: Workspace) -> Outcome {
  for stack in workspace.stacks {
    println!("{}\n", Style::new().underline().paint(stack.to_string()));
    print_stack(stack.tasks());
  }
  Outcome::Success
}

pub fn print_stack(tasks: &Vec<Task>) {
  let mut tw = TabWriter::new(vec![]);
  for task in tasks {
    let text = format!(
      "  {}\t{}\n",
      Style::new().bold().paint(&task.name),
      task.desc
    );
    _ = tw.write(text.as_bytes()).unwrap();
  }
  let bytes = tw.into_inner().unwrap();
  unsafe {
    println!("{}", str::from_utf8_unchecked(&bytes));
  }
}
