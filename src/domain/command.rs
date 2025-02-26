/// all CLI commands that could be run
pub enum Command {
  Run(String),
  Setup,
  FishCompletionSetup,
  FishCompletion,
  Help,
}
