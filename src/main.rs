use std::fmt::Display;
use std::process::{ExitCode, Termination};

mod commands;
mod probes;

/// all commands that could be run
enum Command {
    List,
}

fn parse_cli_args<AS: AsRef<str>>(mut args: impl Iterator<Item = AS>) -> Command {
    match args.next() {
        _ => Command::List,
    }
}

/// a stack that Atalanta knows about
pub trait Stack: Display {
    /// provides all executable tasks for the codebase in the current directory
    fn tasks(&self) -> Result<Vec<Task>, Outcome>;
}

type Stacks = Vec<Box<dyn Stack>>;

/// a task that can be executed
#[derive(Debug, PartialEq)]
pub struct Task {
    /// name of this task, for running it
    pub name: String,
    /// the command to run
    pub cmd: String,
    /// description
    pub desc: String,
}

/// end result of an Atalanta run
pub enum Outcome {
    /// successfully executed the requested command
    CommandExecuted {
        /// the exit code of the executed command
        exit_code: u8,
    },
    /// Atalanta command ran successfully
    Ok,
    /// couldn't determine a stack
    UnknownStack,
    CannotReadFile {
        path: String,
        error: String,
    },
}

impl Termination for Outcome {
    fn report(self) -> ExitCode {
        match self {
            Outcome::CommandExecuted { exit_code } => ExitCode::from(exit_code),
            Outcome::Ok => ExitCode::SUCCESS,
            Outcome::UnknownStack => {
                println!("Error: cannot determine stack");
                ExitCode::FAILURE
            }
            Outcome::CannotReadFile { path, error } => {
                println!("Error: cannot read file {}: {}", path, error);
                ExitCode::FAILURE
            }
        }
    }
}

fn main() -> Outcome {
    let stacks = probes::scan();
    if stacks.len() == 0 {
        return Outcome::UnknownStack;
    };
    match parse_cli_args(std::env::args()) {
        Command::List => commands::list(stacks),
    }
}
