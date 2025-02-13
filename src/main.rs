use cli::Command;
use domain::{Outcome, Workspace};

mod cli;
mod commands;
mod domain;
mod stacks;
mod strings;

fn main() -> Outcome {
  let command = cli::parse(std::env::args());
  execute(command).unwrap_or(Outcome::Success)
}

fn execute(command: Command) -> Option<Outcome> {
  Some(match command {
    Command::List => commands::list(load_workspace()?),
    Command::Run(name) => commands::run(load_workspace()?, name),
    Command::Setup => commands::setup(load_workspace()?),
    Command::FishCompletionSetup => commands::completions::fish::print(),
    Command::FishCompletion => commands::completions::fish::tasks(load_workspace()?),
  })
}

fn load_workspace() -> Option<Workspace> {
  let stacks = stacks::identify();
  if stacks.is_empty() {
    return None;
  };
  Some(Workspace { stacks })
}
