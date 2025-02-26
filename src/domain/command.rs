/// all CLI commands that could be run
pub(crate) enum Command {
  Run(String),
  Setup,
  Install,
  FishCompletionSetup,
  FishCompletion,
  Help,
}
