mod errors;
mod runnables;

pub use errors::UserError;
use std::env;
use std::process::{Command, ExitStatus};

fn main() {
    if let Some(command) = get_command(env::args()) {
        println!("running make");
        let code = run(command).code().unwrap();
        std::process::exit(code);
    } else {
        println!("No command to execute found");
        std::process::exit(1);
    }
}

fn get_command(args: env::Args) -> Option<Command> {
    if let Some(dir) = runnables::makefile::detect() {
        return Some(runnables::makefile::command(args, dir));
    }
    None
}

/// executes the given Runnable with the given args
fn run(mut command: Command) -> ExitStatus {
    command.status().unwrap()
}
