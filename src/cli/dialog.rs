use crate::domain::Task;
use ansi_term::Style;
use crossterm::event::{Event, KeyCode};
use std::io;
use std::io::Write;
use tabwriter::TabWriter;

pub(crate) fn choose_dialog<'a>(tasks: &'a Vec<&Task>) -> &'a Task {
  let mut position = 0;
  let mut stdout = io::stdout();
  loop {
    // print options
    let mut tab_writer = TabWriter::new(vec![]);
    for (i, &task) in tasks.iter().enumerate() {
      let cursor = if i == position { '>' } else { ' ' };
      let text = format!(
        "{cursor} {}\t{}\n",
        Style::new().bold().paint(&task.name),
        task.desc
      );
      tab_writer.write_all(text.as_bytes()).unwrap();
    }
    let bytes = tab_writer.into_inner().unwrap();
    stdout.write_all(&bytes).unwrap();
    stdout.write_all(&[10]).unwrap();
    stdout.flush().unwrap();
    // wait for keyboard input
    let event = crossterm::event::read().unwrap();
    if let Event::Key(key_code) = event {
      match key_code.code {
        KeyCode::Enter => todo!(),
        KeyCode::Up | KeyCode::BackTab => position = cursor_up(position, tasks.len()),
        KeyCode::Down | KeyCode::Tab => position = cursor_down(position, tasks.len()),
        KeyCode::Char(key) => match key {
          'j' => position = cursor_up(position, tasks.len()),
          'k' => position = cursor_down(position, tasks.len()),
          _ => {}
        },
        KeyCode::Esc => std::process::exit(0),
        _ => {}
      }
    }
  }
}

fn cursor_down(cursor: usize, max: usize) -> usize {
  if cursor == (max - 1) { 0 } else { cursor + 1 }
}

fn cursor_up(cursor: usize, max: usize) -> usize {
  if cursor == 0 { max - 1 } else { cursor - 1 }
}
