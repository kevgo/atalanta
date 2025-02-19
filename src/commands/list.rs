use crate::domain::{Outcome, Task, Workspace};
use ansi_term::Style;
use std::io;
use std::io::Write;
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
  let mut tab_writer = TabWriter::new(vec![]);
  for task in tasks {
    let text = format!(
      "  {}\t{}\n",
      Style::new().bold().paint(&task.name),
      task.desc
    );
    tab_writer.write_all(text.as_bytes()).unwrap();
  }
  let bytes = tab_writer.into_inner().unwrap();
  let mut stdout = io::stdout();
  stdout.write_all(&bytes).unwrap();
  stdout.write_all(&[10]).unwrap();
}
