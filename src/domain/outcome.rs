use super::Workspace;
use crate::commands;
use std::process::{ExitCode, Termination};

/// end result of an Atalanta run
pub enum Outcome {
    /// Atalanta has been able to do its thing.
    Success,
    /// A user-provided script didn't succeed
    ScriptFailed {
        /// the exit code to signal when quitting
        exit_code: u8,
    },
    /// Atalanta doesn't know how to set up this workspace
    NoSetup,
    /// couldn't determine a stack
    UnknownStack,
    /// there is no task with the given name
    UnknownTask {
        /// name of the task that we didn't find
        task: String,
        /// copy of the workspace to list all available tasks
        workspace: Workspace,
    },
    /// Atalanta couldn't run an executable defined in a task
    CannotFindExecutable { err: String },
}

impl Termination for Outcome {
    fn report(self) -> ExitCode {
        match self {
            Outcome::Success => ExitCode::SUCCESS,
            Outcome::ScriptFailed { exit_code } => ExitCode::from(exit_code),
            Outcome::UnknownStack => {
                println!("Error: cannot determine stack");
                ExitCode::FAILURE
            }
            Outcome::UnknownTask { task, workspace } => {
                println!("Error: task \"{}\" doesn't exist\n", task);
                commands::list(workspace);
                ExitCode::FAILURE
            }
            Outcome::CannotFindExecutable { err } => {
                println!("Error: cannot find executable: {}", err);
                ExitCode::FAILURE
            }
            Outcome::NoSetup => {
                println!("Warning: I don't know how to set up this workspace");
                ExitCode::FAILURE
            }
        }
    }
}
