use crate::domain::{Stack, Stacks, Tasks};
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

struct RubyBundlerStack {
  tasks: Tasks,
}

impl Display for RubyBundlerStack {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Ruby (Bundler)")
  }
}

impl Stack for RubyBundlerStack {
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
  if Path::new("Cargo.toml").exists() {
    stacks.push(Box::new(RubyBundlerStack {
      tasks: Tasks::new(),
    }));
  }
}
