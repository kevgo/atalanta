use crate::Stacks;

mod makefile;

/// determines the stacks in the current directory
pub fn scan() -> Stacks {
    let mut result = vec![];
    makefile::scan(&mut result);
    result
}
