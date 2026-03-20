use crate::domain::{Stack, Task, Tasks};
use big_s::S;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

struct RubyBundlerStack {
  tasks: Tasks,
}

impl Display for RubyBundlerStack {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Ruby (Bundler)")
  }
}

impl Stack for RubyBundlerStack {
  fn setup(&self) -> Option<Command> {
    let mut command = Command::new("bundle");
    command.arg("install");
    Some(command)
  }

  fn install(&self) -> Option<Command> {
    None
  }

  fn tasks(&self) -> &Tasks {
    &self.tasks
  }
}

pub(crate) fn scan() -> Option<Box<dyn Stack>> {
  if Path::new("Gemfile").exists() && Path::new("Rakefile").exists() {
    return Some(Box::new(RubyBundlerStack {
      tasks: parse_task_list(&load_tasks()),
    }));
  }
  None
}

fn load_tasks() -> String {
  let mut cmd = Command::new("rake");
  cmd.arg("--tasks");
  let output = cmd.output().unwrap();
  String::from_utf8(output.stdout).unwrap()
}

fn parse_task_list(list: &str) -> Tasks {
  let mut result = Tasks::new();
  for line in list.lines() {
    if line.is_empty() {
      continue;
    }
    let Some((name, desc)) = line.split_once('#') else {
      continue;
    };
    if !name.starts_with("rake ") {
      continue;
    }
    let name = name[5..].trim().to_string();
    result.push(Task {
      name: name.clone(),
      cmd: S("bundle"),
      argv: vec![S("exec"), S("rake"), name],
      desc: desc.trim().to_string(),
    });
  }
  result
}

#[cfg(test)]
mod tests {
  use crate::domain::{Task, Tasks};
  use big_s::S;

  #[test]
  fn parse_task_list() {
    let give = "\
rake  build:checksum   # Generate SHA512 checksum of kappamaki-1.0.0.gem into the checksums directory
rake release[remote]  # Create tag v1.0.0 and build and push kappamaki-1.0.0.gem to rubygems.org
rake spec             # Run RSpec code examples\
";
    let have = super::parse_task_list(give);
    let mut want = Tasks::new();
    want.push(Task {
      name: S("build:checksum"),
      cmd: S("bundle"),
      argv: vec![S("exec"), S("rake"), S("build:checksum")],
      desc: S("Generate SHA512 checksum of kappamaki-1.0.0.gem into the checksums directory"),
    });
    want.push(Task {
      name: S("release[remote]"),
      cmd: S("bundle"),
      argv: vec![S("exec"), S("rake"), S("release[remote]")],
      desc: S("Create tag v1.0.0 and build and push kappamaki-1.0.0.gem to rubygems.org"),
    });
    want.push(Task {
      name: S("spec"),
      cmd: S("bundle"),
      argv: vec![S("exec"), S("rake"), S("spec")],
      desc: S("Run RSpec code examples"),
    });
    assert_eq!(have, want);
  }
}
