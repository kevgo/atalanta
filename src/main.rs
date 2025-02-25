mod cli;
mod commands;
mod domain;
mod stacks;

use domain::{Command, Outcome};

fn main() -> Outcome {
  match cli::parse(std::env::args()) {
    Command::Run(name) => commands::run(Workspace::load(), name),
    Command::Setup => commands::setup(Workspace::load()),
    Command::FishCompletionSetup => commands::completions::fish::print(),
    Command::FishCompletion => commands::completions::fish::tasks(stacks::load()),
  }
}
