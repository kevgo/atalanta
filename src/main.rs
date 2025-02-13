use cli::Command;
use domain::{Outcome, Stacks};

mod cli;
mod commands;
mod domain;
mod stacks;
mod strings;

fn main() -> Outcome {
  match cli::parse(std::env::args()) {
    Command::List => commands::list(Stacks::load()),
    Command::Run(name) => commands::run(Stacks::load(), name),
    Command::Setup => commands::setup(Stacks::load()),
    Command::FishCompletionSetup => commands::completions::fish::print(),
    Command::FishCompletion => commands::completions::fish::tasks(Stacks::load()),
  }
}
