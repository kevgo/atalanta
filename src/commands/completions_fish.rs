use crate::domain::{Outcome, Workspace};

pub fn setup() -> Outcome {
    // disable file completions for the entire command
    println!("complete -c a -f");
    // completions for the built-in commands
    println!("complete -c a -a '-s' -d 'set up the codebase'");
    // completions for the tasks in the current directory
    println!("complete -c a -a \"(a --fish-completion)\"");
    Outcome::Success
}

pub fn tasks(workspace: Workspace) -> Outcome {
    for stack in workspace.stacks {
        for task in stack.tasks() {
            println!("{}\t{}", task.name, task.desc);
        }
    }
    Outcome::Success
}
