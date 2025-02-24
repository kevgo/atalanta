/// all CLI commands that could be run
pub enum Command {
  List,
  Run(String),
  Setup,
  FishCompletionSetup,
  FishCompletion,
}
