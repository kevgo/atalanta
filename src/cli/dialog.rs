use crate::domain::Task;
use ansi_term::Style;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use crossterm::{QueueableCommand, cursor};
use std::io::{Stderr, Write};
use std::{io, process};
use tabwriter::TabWriter;

/// displays a dialog that allows the user to select a task to execute
pub(crate) fn select<'a>(tasks: &'a Vec<&Task>) -> &'a Task {
  let mut position = 0;
  let mut aborted = false;
  let mut stderr = io::stderr();
  enable_raw_mode().unwrap();
  stderr.queue(cursor::SavePosition).unwrap();
  stderr.queue(cursor::Hide).unwrap();
  loop {
    clear_output(&mut stderr);
    print_options(&mut stderr, tasks, position);
    // read and handle keyboard input
    let event = crossterm::event::read().unwrap();
    if let Event::Key(key_code) = event {
      if key_code.kind == KeyEventKind::Release {
        continue;
      }
      match key_code.code {
        KeyCode::Down | KeyCode::Tab => position = cursor_down(position, tasks.len()),
        KeyCode::Up | KeyCode::BackTab => position = cursor_up(position, tasks.len()),
        KeyCode::Enter => break,
        KeyCode::Char(key) => match key {
          'j' => position = cursor_down(position, tasks.len()),
          'k' => position = cursor_up(position, tasks.len()),
          'o' => break,
          'q' => {
            aborted = true;
            break;
          }
          _ => {}
        },
        KeyCode::Esc => {
          aborted = true;
          break;
        }
        _ => {}
      }
    }
  }
  clear_output(&mut stderr);
  stderr.queue(cursor::Show).unwrap();
  disable_raw_mode().unwrap();
  if aborted {
    process::exit(0);
  }
  tasks[position]
}

/// removes the output created by print_options
fn clear_output(stderr: &mut Stderr) {
  stderr.queue(cursor::RestorePosition).unwrap();
  stderr.queue(Clear(ClearType::FromCursorDown)).unwrap();
}

/// prints the dialog
fn print_options(stderr: &mut Stderr, tasks: &[&Task], position: usize) {
  let mut tab_writer = TabWriter::new(vec![]);
  for (i, &task) in tasks.iter().enumerate() {
    let cursor = if i == position { '>' } else { ' ' };
    let text = format!(
      "{cursor} {}\t{}\r\n",
      Style::new().bold().paint(&task.name),
      task.desc
    );
    tab_writer.write_all(text.as_bytes()).unwrap();
  }
  let bytes = tab_writer.into_inner().unwrap();
  stderr.write_all(&bytes).unwrap();
  stderr.write_all(&[10]).unwrap();
  stderr.flush().unwrap();
}

fn cursor_down(cursor: usize, max: usize) -> usize {
  if cursor == (max - 1) { 0 } else { cursor + 1 }
}

fn cursor_up(cursor: usize, max: usize) -> usize {
  if cursor == 0 { max - 1 } else { cursor - 1 }
}
