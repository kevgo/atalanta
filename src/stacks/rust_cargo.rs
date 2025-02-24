use crate::domain::Task;
use crate::stacks::Stack;
use big_s::S;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

struct RustCargoStack {
  tasks: Vec<Task>,
}

impl Display for RustCargoStack {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Rust (Cargo)")
  }
}

impl Stack for RustCargoStack {
  fn setup(&self) -> Option<Command> {
    None
  }

  fn tasks(&self) -> &Vec<Task> {
    &self.tasks
  }
}

pub fn scan(stacks: &mut Vec<Box<dyn Stack>>) {
  if !Path::new("Cargo.lock").exists() {
    // TODO: do we really need to verify that this file exists?
    return;
  }
  stacks.push(Box::new(RustCargoStack {
    tasks: vec![
      Task {
        name: S("build"),
        cmd: S("cargo"),
        argv: vec![S("build")],
        desc: S("cargo build"),
      },
      Task {
        name: S("check"),
        cmd: S("cargo"),
        argv: vec![S("check")],
        desc: S("cargo check"),
      },
      Task {
        name: S("test"),
        cmd: S("cargo"),
        argv: vec![S("test")],
        desc: S("cargo test"),
      },
    ],
  }));
}
