use std::env;
use std::process::Command;

mod runnables;

fn main() {
    if let Some(mut command) = find_command(env::args()) {
        let status = command.status().unwrap();
        std::process::exit(status.code().unwrap());
    } else {
        println!("No command to execute found");
        std::process::exit(1);
    }
}

/// provides the command to run
fn find_command(args: env::Args) -> Option<Command> {
    if let Some(dir) = runnables::makefile::detect() {
        return Some(runnables::makefile::command(args, dir));
    }
    None
}
