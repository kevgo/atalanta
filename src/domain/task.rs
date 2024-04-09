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

  /// sort function for sorting Tasks by name
  pub fn sort(a: &Task, b: &Task) -> Ordering {
    a.name.cmp(&b.name)
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
    have.sort_unstable_by(Task::sort);
    assert_eq!(have, want);
  }
}
