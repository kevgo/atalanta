use crate::domain::{Outcome, Workspace};

/// prints Fish commands that set up autocompletion
pub fn print() -> Outcome {
    for command in setup_commands() {
        println!("{command}");
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

/// the commands to set up autocompletion for Fish shell
fn setup_commands() -> Vec<&'static str> {
    vec![
        // disable completing filenames
        "complete -c a -f",
        // complete the built-in commands
        "complete -c a -a '-s' -d 'set up the codebase'",
        // complete the tasks in the current directory
        "complete -c a -a \"(a --fish-completion)\"",
    ]
}
