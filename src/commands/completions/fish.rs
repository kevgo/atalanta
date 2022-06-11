use crate::domain::{Outcome, Workspace};

/// prints Fish commands that set up autocompletion
pub fn print() -> Outcome {
    for command in setup_commands() {
        println!("{}", command);
    }
    Outcome::Success
}

/// used within autocompletion, prints available tasks in an auto-completion compatible format
pub fn tasks(workspace: Workspace) -> Outcome {
    for stack in workspace.stacks {
        for task in stack.tasks() {
            println!("{}\t{}", task.name, task.desc);
        }
    }
    Outcome::Success
}

fn setup_commands() -> Vec<&'static str> {
    vec![
        // disable file completions for the entire command
        "complete -c a -f",
        // completions for the built-in commands
        "complete -c a -a '-s' -d 'set up the codebase'",
        // completions for the tasks in the current directory
        "complete -c a -a \"(a --fish-completion)\"",
    ]
}
