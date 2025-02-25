use super::{Task, Tasks};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::cmp::Ordering;
use std::fmt::Display;
use std::process::Command;

/// a technology stack that Atalanta knows about
pub trait Stack: Display {
  /// provides a Command instance initialized to set up this stack
  fn setup(&self) -> Option<Command>;

  /// Provides all executable tasks for the codebase in the current directory.
  /// This only emits read references. The stack instance should own the task data.
  fn tasks(&self) -> &Tasks;
}

#[derive(Default)]
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

  /// provides all the tasks from all stacks that match the given task name
  pub fn tasks_fuzzy_matching_name(&self, name: &str) -> Vec<&Task> {
    let matcher = SkimMatcherV2::default();
    let mut search_results = vec![];
    for stack in &self.0 {
      for task in stack.tasks() {
        if let Some(score) = matcher.fuzzy_match(&task.name, name) {
          search_results.push(SearchResult { task, score });
        }
      }
    }
    search_results.sort();
    search_results.into_iter().map(|sr| sr.task).collect()
  }
}

impl IntoIterator for Stacks {
  type Item = Box<dyn Stack>;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a Stacks {
  type Item = &'a Box<dyn Stack>;
  type IntoIter = std::slice::Iter<'a, Box<dyn Stack>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

#[derive(Eq, PartialEq)]
struct SearchResult<'a> {
  task: &'a Task,
  score: i64,
}

impl Ord for SearchResult<'_> {
  fn cmp(&self, other: &Self) -> Ordering {
    // compare the other way around here so that highest scores come first
    match other.score.cmp(&self.score) {
      Ordering::Equal => {}
      ord => return ord,
    }
    self.task.name.cmp(&other.task.name)
  }
}

impl PartialOrd for SearchResult<'_> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[cfg(test)]
mod tests {

  mod tasks_fuzzy_matching_name {
    use crate::domain::{Task, Tasks};
    use big_s::S;
    use std::fmt::Display;

    struct TestStack {
      tasks: Tasks,
    }
    impl super::super::Stack for TestStack {
      fn setup(&self) -> Option<std::process::Command> {
        None
      }
      fn tasks(&self) -> &Tasks {
        &self.tasks
      }
    }
    impl Display for TestStack {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("test stack")
      }
    }

    #[test]
    #[allow(clippy::similar_names)]
    fn direct_and_fuzzy_matches_in_two_stacks() {
      let task_1a = Task {
        name: S("install"),
        ..Default::default()
      };
      let task_1b = Task {
        name: S("initialize"),
        ..Default::default()
      };
      let task_1c = Task {
        name: S("intl"),
        ..Default::default()
      };
      let task_1d = Task {
        name: S("foo"),
        ..Default::default()
      };
      let task_2a = Task {
        name: S("internalize"),
        ..Default::default()
      };
      let stack_1 = TestStack {
        tasks: Tasks::from(vec![
          task_1a.clone(),
          task_1b.clone(),
          task_1c.clone(),
          task_1d.clone(),
        ]),
      };
      let stack_2 = TestStack {
        tasks: Tasks::from(vec![task_2a.clone()]),
      };
      let stacks = super::super::Stacks(vec![Box::new(stack_1), Box::new(stack_2)]);
      let have: Vec<String> = stacks
        .tasks_fuzzy_matching_name("intl")
        .into_iter()
        .map(|task| task.name.clone())
        .collect();
      let want = vec![S("intl"), S("internalize"), S("install"), S("initialize")];
      assert_eq!(have, want);
    }

    #[test]
    #[allow(clippy::similar_names)]
    fn no_matches() {
      let stacks = super::super::Stacks(vec![
        Box::new(TestStack {
          tasks: Tasks::from(vec![Task {
            name: S("foo"),
            ..Default::default()
          }]),
        }),
        Box::new(TestStack {
          tasks: Tasks::from(vec![Task {
            name: S("bar"),
            ..Default::default()
          }]),
        }),
      ]);
      let have: Vec<String> = stacks
        .tasks_fuzzy_matching_name("intl")
        .into_iter()
        .map(|task| task.name.clone())
        .collect();
      let want: Vec<String> = vec![];
      assert_eq!(have, want);
    }
  }
}
