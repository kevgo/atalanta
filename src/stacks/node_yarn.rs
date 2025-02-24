use super::node_npm::{PackageJson, load_package_json};
use crate::domain::{Stack, Stacks, Task, Tasks};
use big_s::S;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

struct NodeYarnStack {
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

  fn tasks(&self) -> &Tasks {
    &self.tasks
  }
}

pub fn scan(stacks: &mut Stacks, mut dir: &Path) {
  let Some(package_json) = load_package_json() else {
    return;
  };
  loop {
    let yarn_lock = dir.join("yarn.lock");
    if yarn_lock.exists() {
      stacks.push(Box::new(NodeYarnStack {
        tasks: parse_scripts(package_json),
      }));
      return;
    }
    match dir.parent() {
      Some(parent) => dir = parent,
      None => return,
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
