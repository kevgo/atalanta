use super::node_npm::{PackageJson, load_package_json};
use crate::domain::{Stack, Task, Tasks};
use big_s::S;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

pub(crate) struct NodeYarnStack {
  tasks: Tasks,
}

impl Display for NodeYarnStack {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Node.JS (yarn)")
  }
}

impl Stack for NodeYarnStack {
  fn setup(&self) -> Option<Command> {
    let mut cmd = Command::new("yarn");
    cmd.arg("install");
    Some(cmd)
  }

  fn install(&self) -> Option<Command> {
    None
  }

  fn tasks(&self) -> &Tasks {
    &self.tasks
  }
}

pub(crate) fn scan(mut dir: &Path) -> Option<Box<dyn Stack>> {
  let package_json = load_package_json()?;
  loop {
    let yarn_lock = dir.join("yarn.lock");
    if yarn_lock.exists() {
      return Some(Box::new(NodeYarnStack {
        tasks: parse_scripts(package_json),
      }));
    }
    match dir.parent() {
      Some(parent) => dir = parent,
      None => return None,
    }
  }
}

fn parse_scripts(package_json: PackageJson) -> Tasks {
  let mut result = Tasks::new();
  if let Some(scripts) = package_json.scripts {
    for (key, _value) in scripts {
      result.push(Task {
        name: key.clone(),
        cmd: S("yarn"),
        argv: vec![S("--silent"), S("run"), key],
        desc: String::new(),
      });
    }
    result.sort();
  }
  result
}
