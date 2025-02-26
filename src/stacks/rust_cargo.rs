use crate::domain::{Stack, Stacks, Tasks};
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

struct RustCargoStack {
  tasks: Tasks,
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

  fn install(&self) -> Option<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(vec!["install", "--path", "."]);
    Some(cmd)
  }

  fn tasks(&self) -> &Tasks {
    &self.tasks
  }
}

pub(crate) fn scan(stacks: &mut Stacks) {
  if !Path::new("Cargo.lock").exists() {
    // TODO: do we really need to verify that this file exists?
    return;
  }
  stacks.push(Box::new(RustCargoStack {
    tasks: Tasks::new(),
  }));
}
