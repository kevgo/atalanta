use crate::Stacks;

mod makefile;
mod node_npm;

/// determines the stacks in the current directory
pub fn scan() -> Stacks {
    let mut result = vec![];
    makefile::scan(&mut result);
    node_npm::scan(&mut result);
    result
}
