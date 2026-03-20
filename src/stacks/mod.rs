//! This module contains the various technology stacks that Atalanta can run.

mod makefile;
mod node_npm;
mod node_yarn;
mod ruby_bundler;
mod rust_cargo;

use crate::domain::{Stack, Stacks};
use std::env;

/// determines the existing stacks
pub(crate) fn load() -> Stacks {
  let cwd = env::current_dir().unwrap();
  let stacks = vec![
    makefile::scan(),
    node_npm::scan(&cwd),
    node_yarn::scan(&cwd),
    ruby_bundler::scan(),
    rust_cargo::scan(),
  ];
  let stacks: Vec<Box<dyn Stack>> = stacks.into_iter().flatten().collect();
  Stacks::from(stacks)
}
