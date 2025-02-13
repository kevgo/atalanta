use crate::domain::Stack;

mod makefile;
mod node_npm;
mod node_yarn;
mod rust_cargo;

/// determines the stacks in the current workspace
pub fn identify() -> Vec<Box<dyn Stack>> {
  let mut result = vec![];
  makefile::scan(&mut result);
  node_npm::scan(&mut result);
  node_yarn::scan(&mut result);
  rust_cargo::scan(&mut result);
  result
}
