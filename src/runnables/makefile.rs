use super::Runnable;
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// detects a Makefile in the current or a parent directory
pub fn detect() -> Option<Box<dyn Runnable>> {
    let dir = env::current_dir().expect("Cannot get current directory");
    loop {
        let mut path = dir.join("Makefile");
        if path.is_file() {
            return Some(Box::new(Makefile { path }));
        }
        if !path.pop() || !path.pop() {
            return None;
        }
    }
}

pub struct Makefile {
    /// path of the Makefile
    path: PathBuf,
}

impl Runnable for Makefile {
    fn command(&self, args: env::Args) -> Command {
        let mut command = Command::new("make");
        command.args(args);
        command.current_dir(&self.path);
        command
    }
}
