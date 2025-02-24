use super::{Task, Tasks};
use crate::strings;
use std::fmt::Display;
use std::process::Command;
use std::vec::IntoIter;

/// a technology stack that Atalanta knows about
pub trait Stack: Display {
  /// provides the command to set up this stack (binary, argv)
  fn setup(&self) -> Option<Command>;

  /// Provides all executable tasks for the codebase in the current directory.
  /// This only emits read references. The stack instance should own the task data.
  fn tasks(&self) -> &Tasks;
}

pub struct Stacks(Vec<Box<dyn Stack>>);

impl Stacks {
  pub fn new() -> Self {
    Self(vec![])
  }

  pub fn push(&mut self, stack: Box<dyn Stack>) {
    self.0.push(stack);
  }

  pub fn task_with_name(&self, name: &str) -> Option<&Task> {
    for stack in &self.0 {
      let task = stack.tasks().with_name(name);
      if task.is_some() {
        return task;
      }
    }
    None
  }

  pub fn tasks_matching_name(&self, name: &str) -> Vec<&str> {
    let mut task_names = vec![];
    for stack in &self.0 {
      for task in stack.tasks() {
        task_names.push(task.name.as_str());
      }
    }
    strings::matching(name, task_names)
  }
}

impl IntoIterator for Stacks {
  type Item = Box<dyn Stack>;
  type IntoIter = IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

pub struct Stacks(Vec<Box<dyn Stack>>);

impl Stacks {}
