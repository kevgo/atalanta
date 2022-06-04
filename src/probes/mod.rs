use std::fmt::Display;

mod makefile;

/// a stack that Run knows about
pub enum Stack {
    /// a Makefile
    Makefile,
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Stack::Makefile => "Makefile",
        })
    }
}

impl Stack {
    pub fn commands(&self) -> Vec<Task> {
        vec![]
    }
}

/// a task that can be executed
pub struct Task {
    pub cmd: String,
    pub desc: Option<String>,
}

/// tries to determine the stack used in the current directory
pub fn scan() -> Option<Stack> {
    let make = makefile::scan();
    if make.is_some() {
        return make;
    }
    None
}
