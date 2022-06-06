/// a task that can be executed
#[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Task {
    /// name of this task, for identifying it via the CLI
    pub name: String,
    /// the binary to run
    pub cmd: String,
    /// command-line arguments for the binary
    pub argv: Vec<String>,
    /// optional description
    pub desc: Option<String>,
}
