use crate::domain::Stack;
use std::env;

mod makefile;
mod node_npm;
mod node_yarn;
mod rust_cargo;

/// determines the stacks in the current workspace
pub fn load() -> Vec<Box<dyn Stack>> {
  let mut result = vec![];
  let cwd = env::current_dir().unwrap();
  makefile::scan(&mut result);
  node_npm::scan(&mut result);
  node_yarn::scan(&mut result, &cwd);
  rust_cargo::scan(&mut result);
  result
}
