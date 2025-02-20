use crate::domain::{Stack, Task};
use big_s::S;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;
use std::process::Command;

struct NodeNpmStack {
  tasks: Vec<Task>,
}

impl Display for NodeNpmStack {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Node.JS (npm)")
  }
}

impl Stack for NodeNpmStack {
  fn setup(&self) -> Option<Command> {
    let mut command = Command::new("npm");
    command.arg("install");
    Some(command)
  }

  fn tasks(&self) -> &Vec<Task> {
    &self.tasks
  }
}

#[derive(Deserialize)]
pub struct PackageJson {
  pub scripts: Option<HashMap<String, String>>,
}

pub fn scan(stacks: &mut Vec<Box<dyn Stack>>) {
  if !Path::new("package-lock.json").exists() {
    return;
  }
  let Some(package_json) = load_package_json() else {
    return;
  };
  stacks.push(Box::new(NodeNpmStack {
    tasks: parse_scripts(package_json),
  }));
}

pub fn load_package_json() -> Option<PackageJson> {
  let file = match File::open("package.json") {
    Ok(file) => file,
    Err(e) => match e.kind() {
      ErrorKind::NotFound => return None,
      e => {
        println!("Warning: Cannot read file \"package.json\": {e}");
        return None;
      }
    },
  };
  let reader = BufReader::new(file);
  match serde_json::from_reader(reader) {
    Ok(content) => Some(content),
    Err(e) => {
      println!("Warning: file \"package.json\" has an invalid structure: {e}",);
      None
    }
  }
}

fn parse_scripts(package_json: PackageJson) -> Vec<Task> {
  let mut result = vec![];
  if let Some(scripts) = package_json.scripts {
    for (key, _value) in scripts {
      result.push(Task {
        name: key.clone(),
        cmd: S("npm"),
        argv: vec![S("run"), key, S("--silent")],
        desc: String::new(),
      });
    }
    result.sort_unstable_by(Task::sort);
  }
  result
}
