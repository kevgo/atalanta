mod cli;
mod commands;
mod domain;
mod stacks;
mod strings;

use domain::{Command, Outcome, Workspace};

fn main() -> Outcome {
  match cli::parse(std::env::args()) {
    Command::Run(name) => commands::run(Workspace::load(), name),
    Command::Setup => commands::setup(Workspace::load()),
    Command::FishCompletionSetup => commands::completions::fish::print(),
    Command::FishCompletion => commands::completions::fish::tasks(Workspace::load()),
  }
}
