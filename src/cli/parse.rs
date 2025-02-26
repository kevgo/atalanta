use crate::domain::Command;
use std::env::Args;

pub(crate) fn parse(mut args: Args) -> Command {
  args.next(); // skip the binary name
  match args.next() {
    Some(cmd) if cmd == "-h" => Command::Help,
    Some(cmd) if cmd == "--help" => Command::Help,
    Some(cmd) if cmd == "-s" => Command::Setup,
    Some(cmd) if cmd == "--setup" => Command::Setup,
    Some(cmd) if cmd == "--fish-completion" => Command::FishCompletion,
    Some(cmd) if cmd == "--setup-fish-completions" => Command::FishCompletionSetup,
    Some(cmd) => Command::Run(cmd),
    None => Command::Run(String::new()),
  }
}
