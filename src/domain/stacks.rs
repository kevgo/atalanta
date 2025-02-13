use super::{Stack, Task};
use crate::{stacks, strings};
use std::vec::IntoIter;

pub struct Stacks {
  values: Vec<Box<dyn Stack>>,
}

impl Stacks {
  /// loads the workspace in the current working directory
  pub fn load() -> Stacks {
    Stacks {
      values: stacks::identify(),
    }
  }

  pub fn task_with_name(&self, name: &str) -> Option<&Task> {
    for stack in &self.values {
      let task = stack.task_with_name(name);
      if task.is_some() {
        return task;
      }
    }
    None
  }

  pub fn tasks_matching_name(&self, name: &str) -> Vec<&str> {
    let mut task_names = vec![];
    for stack in &self.values {
      for task in stack.tasks() {
        task_names.push(task.name.as_str());
      }
    }
    strings::matching(name, task_names)
  }
}

impl IntoIterator for Stacks {
  type Item = Box<dyn Stack>;

  type IntoIter = IntoIter<Box<dyn Stack>>;

  fn into_iter(self) -> Self::IntoIter {
    self.values.into_iter()
  }
}
