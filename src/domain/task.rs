use std::process::Command;

/// a task that can be executed
#[derive(Debug, PartialEq)]
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

impl Task {
    pub fn command(&self) -> Command {
        let mut cmd = Command::new(&self.cmd);
        cmd.args(&self.argv);
        cmd
    }
}
