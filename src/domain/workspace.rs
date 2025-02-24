use std::cmp::Ordering;

use super::{Stack, Task};
use crate::stacks;
use fuzzy_matcher::{self, FuzzyMatcher};

pub struct Workspace {
  pub stacks: Vec<Box<dyn Stack>>,
}

impl Workspace {
  /// loads the workspace in the current working directory
  pub fn load() -> Workspace {
    Workspace {
      stacks: stacks::load(),
    }
  }

  pub fn task_with_name(&self, name: &str) -> Option<&Task> {
    for stack in &self.stacks {
      let task = stack.task_with_name(name);
      if task.is_some() {
        return task;
      }
    }
    None
  }

  pub fn tasks_matching_name(&self, name: &str) -> Vec<&str> {
    let mut matches = vec![];
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
    for stack in &self.stacks {
      for task in stack.tasks() {
        if let Some(score) = matcher.fuzzy_match(&task.name, name) {
          matches.push(SearchResult { task, score });
        }
      }
    }
    matches.sort();
  }
}

#[derive(Eq, PartialEq)]
struct SearchResult<'a> {
  task: &'a Task,
  score: i64,
}

impl<'a> Ord for SearchResult<'a> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match self.score.cmp(&other.score) {
      Ordering::Equal => {}
      ord => return ord,
    }
    self.task.name.cmp(&other.task.name)
  }
}

impl<'a> PartialOrd for SearchResult<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
