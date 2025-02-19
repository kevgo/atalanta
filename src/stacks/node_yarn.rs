use super::node_npm::{load_package_json, PackageJson};
use crate::domain::{Stack, Task};
use big_s::S;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

struct NodeYarnStack {
  tasks: Vec<Task>,
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

  fn tasks(&self) -> &Vec<Task> {
    &self.tasks
  }
}

pub fn scan(stacks: &mut Vec<Box<dyn Stack>>, mut dir: &Path) {
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

fn parse_scripts(package_json: PackageJson) -> Vec<Task> {
  let mut result = vec![];
  if let Some(scripts) = package_json.scripts {
    for (key, value) in scripts {
      result.push(Task {
        name: key.clone(),
        cmd: S("yarn"),
        argv: vec![S("--silent"), S("run"), key],
        desc: value,
      });
    }
    result.sort_unstable_by(Task::sort);
  }
  result
}
