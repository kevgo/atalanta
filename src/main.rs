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
    FishCompletionInstall,
    FishCompletionSetup,
    FishCompletion,
}

fn parse_cli_args(mut args: Args) -> Command {
    args.next(); // skip the binary name
    match args.next() {
        Some(cmd) if cmd == "-s" => Command::Setup,
        Some(cmd) if cmd == "--fish-completion" => Command::FishCompletion,
        Some(cmd) if cmd == "--install-fish-completions" => Command::FishCompletionInstall,
        Some(cmd) if cmd == "--print-fish-completions" => Command::FishCompletionSetup,
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
        Command::FishCompletionInstall => commands::completions::fish::install(),
        Command::FishCompletionSetup => commands::completions::fish::print(),
        Command::FishCompletion => commands::completions::fish::tasks(workspace),
    }
}
