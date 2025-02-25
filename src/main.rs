mod cli;
mod commands;
mod domain;
mod stacks;

use domain::{Command, Outcome};

fn main() -> Outcome {
  match cli::parse(std::env::args()) {
    Command::List => commands::list(stacks::load()),
    Command::Run(name) => commands::run(stacks::load(), name),
    Command::Setup => commands::setup(stacks::load()),
    Command::FishCompletionSetup => commands::completions::fish::print(),
    Command::FishCompletion => commands::completions::fish::tasks(stacks::load()),
  }
}
