use std::env;
use std::process::Command;

pub mod makefile;

/// provides the command to run
pub fn find(args: env::Args) -> Option<Command> {
    if let Some(dir) = makefile::detect() {
        return Some(makefile::command(args, dir));
    }
    None
}
