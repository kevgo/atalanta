use crate::domain::{Outcome, Stacks, Task};
use ansi_term::Style;
use std::io::Write;
use std::str;
use tabwriter::TabWriter;

/// lists all available commands
pub fn list(workspace: Stacks) -> Outcome {
  for stack in workspace {
    println!("{}\n", Style::new().underline().paint(stack.to_string()));
    print_stack(stack.tasks());
  }
  Outcome::Success
}

pub fn print_stack(tasks: &Vec<Task>) {
  let mut tw = TabWriter::new(vec![]);
  if tasks.is_empty() {
    println!("  (no tasks found)");
    return;
  }
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
