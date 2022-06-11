use crate::domain::{Outcome, Workspace};
use std::fs;

/// prints Fish commands that set up autocompletion
pub fn setup() -> Outcome {
    for command in setup_commands() {
        println!("{}", command);
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

/// used within autocompletion, prints available tasks in an auto-completion compatible format
pub fn tasks(workspace: Workspace) -> Outcome {
    for stack in workspace.stacks {
        for task in stack.tasks() {
            println!("{}\t{}", task.name, task.desc);
        }
    }
    Outcome::Success
}

pub fn install() -> Outcome {
    let path = "~/.config/fish/completions/atalanta.fish";
    let result = fs::write(&path, setup_commands().join("\n"));
    match result {
        Ok(_) => Outcome::Success,
        Err(e) => Outcome::CannotWriteFile {
            path,
            error: e.to_string(),
        },
    }
}
