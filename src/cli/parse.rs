use crate::domain::Command;
use std::env::Args;

pub fn parse(mut args: Args) -> Command {
  args.next(); // skip the binary name
  match args.next() {
    Some(cmd) if cmd == "-s" => Command::Setup,
    Some(cmd) if cmd == "--setup" => Command::Setup,
    Some(cmd) if cmd == "--fish-completion" => Command::FishCompletion,
    Some(cmd) if cmd == "--print-fish-completions" => Command::FishCompletionSetup,
    Some(cmd) => Command::Run(cmd),
    None => Command::List,
  }
}
