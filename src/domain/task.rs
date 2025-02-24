use std::cmp::Ordering;
use std::process::Command;

/// a task that can be executed
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Task {
  /// name of this task, for identifying it via the CLI
  pub name: String,
  /// the binary to run
  pub cmd: String,
  /// command-line arguments for the binary
  pub argv: Vec<String>,
  /// optional description
  pub desc: String,
}

impl Task {
  /// provides a fully configured `Command` that executes this `Task`
  pub fn command(&self) -> Command {
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
      core::cmp::Ordering::Equal => {}
      ord => return ord,
    }
    match self.desc.cmp(&other.desc) {
      core::cmp::Ordering::Equal => {}
      ord => return ord,
    }
    match self.cmd.cmp(&other.cmd) {
      core::cmp::Ordering::Equal => {}
      ord => return ord,
    }
    self.argv.cmp(&other.argv)
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
    have.sort_unstable();
    assert_eq!(have, want);
  }
}
