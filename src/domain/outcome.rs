use super::Workspace;
use crate::commands;
use std::process::{ExitCode, Termination};

/// end result of an Atalanta run
pub enum Outcome {
    /// Atalanta has been able to do its thing
    Success {
        /// the exit code to signal when quitting
        exit_code: u8,
    },
    /// Atalanta doesn't know how to set up this workspace
    UnknownSetup,
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
    CannotRunExecutable { err: String },
}

impl Termination for Outcome {
    fn report(self) -> ExitCode {
        match self {
            Outcome::Success { exit_code } => ExitCode::from(exit_code),
            Outcome::UnknownStack => {
                println!("Error: cannot determine stack");
                ExitCode::FAILURE
            }
            Outcome::UnknownTask { task, workspace } => {
                println!("Error: task \"{}\" doesn't exist\n", task);
                commands::list(workspace);
                ExitCode::FAILURE
            }
            Outcome::CannotRunExecutable { err } => {
                println!("Error: cannot run executable: {}", err);
                ExitCode::FAILURE
            }
            Outcome::UnknownSetup => {
                println!("Warning: I don't know how to set up this workspace");
                ExitCode::FAILURE
            }
        }
    }
}
