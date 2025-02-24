use super::{Stacks, Tasks};
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
  /// Atalanta doesn't know how to set up this stack
  NoSetup,
  /// more than one task matches the shortcut provided by the user
  TooManyTaskMatches { tasks: Tasks },
  /// there is no task with the given name
  UnknownTask {
    /// name of the task that we didn't find
    task: String,
    /// all available stacks, for listing the available tasks
    stacks: Stacks,
  },
  /// Atalanta couldn't run an executable defined in a task
  CannotFindExecutable { err: String },
}

impl<'a> Termination for Outcome {
  fn report(self) -> ExitCode {
    match self {
      Outcome::Success => ExitCode::SUCCESS,
      Outcome::ScriptFailed { exit_code } => ExitCode::from(exit_code),
      Outcome::UnknownTask { task, stacks } => {
        println!("Error: task \"{task}\" doesn't exist\n");
        commands::list(stacks);
        ExitCode::FAILURE
      }
      Outcome::CannotFindExecutable { err } => {
        println!("Error: cannot find executable: {err}");
        ExitCode::FAILURE
      }
      Outcome::NoSetup => {
        println!("Warning: I don't know how to set up this stack");
        ExitCode::FAILURE
      }
      Outcome::TooManyTaskMatches { tasks } => {
        println!("Multiple matches:");
        commands::list::print_stack(tasks.as_ref());
        ExitCode::FAILURE
      }
    }
  }
}
