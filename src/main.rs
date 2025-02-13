use cli::Command;
use domain::{Outcome, Workspace};

mod cli;
mod commands;
mod domain;
mod stacks;
mod strings;

fn main() -> Outcome {
  match cli::parse(std::env::args()) {
    Command::List => commands::list(Workspace::load()),
    Command::Run(name) => commands::run(Workspace::load(), name),
    Command::Setup => commands::setup(Workspace::load()),
    Command::FishCompletionSetup => commands::completions::fish::print(),
    Command::FishCompletion => commands::completions::fish::tasks(Workspace::load()),
  }
}
