use domain::{Outcome, Workspace};
use std::env::Args;

mod commands;
mod domain;
mod stacks;
mod strings;

/// all commands that could be run
enum Command {
    List,
    Run(String),
    Setup,
}

fn parse_cli_args(mut args: Args) -> Command {
    // skip the binary name
    args.next();
    match args.next() {
        Some(cmd) if cmd == "-s" => Command::Setup,
        Some(cmd) => Command::Run(cmd),
        None => Command::List,
    }
}

fn main() -> Outcome {
    let stacks = stacks::identify();
    if stacks.is_empty() {
        return Outcome::UnknownStack;
    };
    let workspace = Workspace { stacks };
    match parse_cli_args(std::env::args()) {
        Command::List => commands::list(workspace),
        Command::Run(name) => commands::run(workspace, name),
        Command::Setup => commands::setup(workspace),
    }
}
