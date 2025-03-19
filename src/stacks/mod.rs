//! This module contains the various technology stacks that Atalanta can run.

mod makefile;
mod node_npm;
mod node_yarn;
mod ruby_bundler;
mod rust_cargo;

use crate::domain::Stacks;
use std::env;

/// determines the existing stacks
pub(crate) fn load() -> Stacks {
  let mut result = Stacks::new();
  let cwd = env::current_dir().unwrap();
  makefile::scan(&mut result);
  node_npm::scan(&mut result, &cwd);
  node_yarn::scan(&mut result, &cwd);
  ruby_bundler::scan(&mut result);
  rust_cargo::scan(&mut result);
  result
}
