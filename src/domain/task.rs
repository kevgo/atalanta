use std::cmp::Ordering;
use std::process::Command;

/// a task that can be executed
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Task {
  /// name of this task, for identifying it via the CLI
  pub(crate) name: String,
  /// the binary to run
  pub(crate) cmd: String,
  /// command-line arguments for the binary
  pub(crate) argv: Vec<String>,
  /// optional description
  pub(crate) desc: String,
}

impl Task {
  /// provides a fully configured `Command` that executes this `Task`
  pub(crate) fn command(&self) -> Command {
    let mut cmd = Command::new(&self.cmd);
    cmd.args(&self.argv);
    cmd
  }
}

impl PartialOrd for Task {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Task {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.name.cmp(&other.name) {
      Ordering::Equal => {}
      ord => return ord,
    }
    match self.desc.cmp(&other.desc) {
      Ordering::Equal => {}
      ord => return ord,
    }
    match self.cmd.cmp(&other.cmd) {
      Ordering::Equal => {}
      ord => return ord,
    }
    self.argv.cmp(&other.argv)
  }
}

#[derive(Default)]
pub(crate) struct Tasks(Vec<Task>);

impl Tasks {
  pub(crate) fn new() -> Self {
    Self(vec![])
  }

  pub(crate) fn push(&mut self, task: Task) {
    self.0.push(task);
  }

  pub(crate) fn sort(&mut self) {
    self.0.sort();
  }
}

impl<'a> IntoIterator for &'a Tasks {
  type Item = &'a Task;
  type IntoIter = std::slice::Iter<'a, Task>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl From<Vec<Task>> for Tasks {
  fn from(tasks: Vec<Task>) -> Self {
    Self(tasks)
  }
}

impl From<Vec<&Task>> for Tasks {
  fn from(tasks: Vec<&Task>) -> Self {
    Self(tasks.into_iter().map(ToOwned::to_owned).collect())
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::Task;
  use big_s::S;

  #[test]
  fn sort() {
    let mut have = vec![
      Task {
        name: S("task 3 name"),
        cmd: S("task 3 cmd"),
        argv: vec![],
        desc: S(""),
      },
      Task {
        name: S("task 2 name"),
        cmd: S("task 2 cmd"),
        argv: vec![],
        desc: S(""),
      },
      Task {
        name: S("task 1 name"),
        cmd: S("task 1 cmd"),
        argv: vec![],
        desc: S(""),
      },
    ];
    let want = vec![
      Task {
        name: S("task 1 name"),
        cmd: S("task 1 cmd"),
        argv: vec![],
        desc: S(""),
      },
      Task {
        name: S("task 2 name"),
        cmd: S("task 2 cmd"),
        argv: vec![],
        desc: S(""),
      },
      Task {
        name: S("task 3 name"),
        cmd: S("task 3 cmd"),
        argv: vec![],
        desc: S(""),
      },
    ];
    have.sort();
    assert_eq!(have, want);
  }
}
