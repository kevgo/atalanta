//! This module contains the various technology stacks that Atalanta can run.

mod makefile;
mod node_npm;
mod node_yarn;
mod rust_cargo;

use crate::domain::Stacks;
use std::env;

/// determines the existing stacks
pub(crate) fn load() -> Stacks {
  let cwd = env::current_dir().unwrap();
  let make_stack = makefile::scan();
  let node_npm_stack = node_npm::scan(&cwd);
  let node_yarn_stack = node_yarn::scan(&cwd);
  let rust_stack = rust_cargo::scan();
  let mut result = Stacks::new();
  if let Some(stack) = make_stack {
    result.push(Box::new(stack));
  }
  if let Some(stack) = node_npm_stack {
    result.push(Box::new(stack));
  }
  if let Some(stack) = node_yarn_stack {
    result.push(Box::new(stack));
  }
  if let Some(stack) = rust_stack {
    result.push(Box::new(stack));
  }
  result
}
