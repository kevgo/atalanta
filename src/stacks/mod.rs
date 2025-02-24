use crate::domain::Task;
use std::env;
use std::fmt::Display;
use std::process::Command;

mod makefile;
mod node_npm;
mod node_yarn;
mod rust_cargo;

/// a technology stack that Atalanta knows about
pub trait Stack: Display {
  /// provides the command to set up this stack (binary, argv)
  fn setup(&self) -> Option<Command>;

  /// Provides all executable tasks for the codebase in the current directory.
  /// This only emits read references. The stack instance should own the task data.
  fn tasks(&self) -> &Vec<Task>;

  /// provides the task with the given name
  fn task_with_name(&self, name: &str) -> Option<&Task> {
    self.tasks().iter().find(|task| task.name == name)
  }
}

/// determines the stacks in the current workspace
pub fn load() -> Vec<Box<dyn Stack>> {
  let mut result = vec![];
  let cwd = env::current_dir().unwrap();
  makefile::scan(&mut result);
  node_npm::scan(&mut result);
  node_yarn::scan(&mut result, &cwd);
  rust_cargo::scan(&mut result);
  result
}
