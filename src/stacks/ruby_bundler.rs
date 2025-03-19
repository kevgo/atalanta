use crate::domain::{Stack, Stacks, Tasks};
use std::fmt::Display;
use std::io;
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

pub(crate) fn scan(stacks: &mut Stacks) {
  if Path::new("Gemfile").exists() && Path::new("Rakefile").exists() {
    stacks.push(Box::new(RubyBundlerStack {
      tasks: parse_task_list(&determine_tasks()),
    }));
  }
}

fn determine_tasks() -> String {
  let mut cmd = Command::new("rake");
  cmd.arg("--tasks");
  let output = cmd.output().unwrap();
  String::from_utf8(output.stdout).unwrap()
}

fn parse_task_list(list: &str) -> Tasks {
  Tasks::new()
}

#[cfg(test)]
mod tests {
  use crate::domain::{Task, Tasks};
  use big_s::S;

  #[test]
  fn parse_task_list() {
    let give = "\
rake build            # Build kappamaki-1.0.0.gem into the pkg directory
rake build:checksum   # Generate SHA512 checksum of kappamaki-1.0.0.gem into the checksums directory
rake clean            # Remove any temporary products
rake clobber          # Remove any generated files
rake install          # Build and install kappamaki-1.0.0.gem into system gems
rake install:local    # Build and install kappamaki-1.0.0.gem into system gems without network access
rake lint             # Run linter
rake release[remote]  # Create tag v1.0.0 and build and push kappamaki-1.0.0.gem to rubygems.org
rake spec             # Run RSpec code examples
";
    let have = super::parse_task_list(give);
    let want = Tasks::new();
    want.push(Task {
      name: S("build"),
      cmd: S("bundle"),
      argv: vec![S("exec"), S("rake"), S("build")],
      desc: S(""),
    });
  }
}
