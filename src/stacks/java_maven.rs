use crate::domain::{Stack, Task, Tasks};
use big_s::S;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

struct JavaMavenStack {
  tasks: Tasks,
}

impl Display for JavaMavenStack {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Java (Maven)")
  }
}

impl Stack for JavaMavenStack {
  fn setup(&self) -> Option<Command> {
    let mut cmd = Command::new("mvn");
    cmd.arg("dependency:go-offline");
    Some(cmd)
  }

  fn install(&self) -> Option<Command> {
    let mut cmd = Command::new("mvn");
    cmd.arg("install");
    Some(cmd)
  }

  fn tasks(&self) -> &Tasks {
    &self.tasks
  }
}

pub(crate) fn scan() -> Option<Box<dyn Stack>> {
  if Path::new("pom.xml").exists() {
    return Some(Box::new(JavaMavenStack { tasks: tasks() }));
  }
  None
}

/// provides the fixed Maven build-lifecycle tasks, in lifecycle order
fn tasks() -> Tasks {
  let mut result = Tasks::new();
  for (name, desc) in [
    ("validate", "check the project is correct"),
    ("compile", "compile source code"),
    ("test", "run unit tests"),
    ("package", "bundle into JAR/WAR"),
    ("verify", "run integration tests & checks"),
  ] {
    result.push(Task {
      name: S(name),
      cmd: S("mvn"),
      argv: vec![S(name)],
      desc: S(desc),
    });
  }
  result
}

#[cfg(test)]
mod tests {
  use crate::domain::{Task, Tasks};
  use big_s::S;

  #[test]
  fn tasks() {
    let have = super::tasks();
    let mut want = Tasks::new();
    want.push(Task {
      name: S("validate"),
      cmd: S("mvn"),
      argv: vec![S("validate")],
      desc: S("check the project is correct"),
    });
    want.push(Task {
      name: S("compile"),
      cmd: S("mvn"),
      argv: vec![S("compile")],
      desc: S("compile source code"),
    });
    want.push(Task {
      name: S("test"),
      cmd: S("mvn"),
      argv: vec![S("test")],
      desc: S("run unit tests"),
    });
    want.push(Task {
      name: S("package"),
      cmd: S("mvn"),
      argv: vec![S("package")],
      desc: S("bundle into JAR/WAR"),
    });
    want.push(Task {
      name: S("verify"),
      cmd: S("mvn"),
      argv: vec![S("verify")],
      desc: S("run integration tests & checks"),
    });
    pretty::assert_eq!(have, want);
  }
}
