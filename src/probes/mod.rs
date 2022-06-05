use crate::Stacks;

mod makefile;
mod node_npm;
mod node_yarn;

/// determines the stacks in the current directory
pub fn scan() -> Stacks {
    let mut result = vec![];
    makefile::scan(&mut result);
    node_npm::scan(&mut result);
    node_yarn::scan(&mut result);
    result
}
