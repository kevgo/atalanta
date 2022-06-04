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

/// end result of an Atalanta run
pub enum Outcome {
    /// successfully executed the requested command
    CommandExecuted {
        /// the exit code of the executed command
        exit_code: u8,
    },
    /// Atalanta command ran successfully
    Ok,
    UnknownStack,
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
        }
    }
}

fn main() -> Outcome {
    let stack = match probes::scan() {
        Some(stack) => stack,
        None => return Outcome::UnknownStack,
    };
    match parse_cli_args(std::env::args()) {
        Command::List => commands::list(stack),
    }
}
