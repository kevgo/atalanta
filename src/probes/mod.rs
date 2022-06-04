use std::fmt::Display;

mod makefile;

/// a stack that Atalanta knows about
pub trait Stack: Display {
    /// provides all executable tasks for the codebase in the current directory
    fn tasks(&self) -> Vec<Task>;
}

/// a task that can be executed
pub struct Task {
    /// name of this task, for running it
    pub name: String,
    /// the command to run
    pub cmd: String,
    /// optional description
    pub desc: Option<String>,
}

/// tries to determine the stack used in the current directory
pub fn scan() -> Option<impl Stack> {
    let make = makefile::scan();
    if make.is_some() {
        return make;
    }
    None
}
