use std::env;
use std::path::PathBuf;
use std::process::Command;

/// detects a Makefile in the current or a parent directory
pub fn detect() -> Option<PathBuf> {
    let dir = env::current_dir().expect("Cannot get current directory");
    loop {
        let mut path = dir.join("Makefile");
        if path.is_file() {
            path.pop();
            return Some(path);
        }
        if !path.pop() || !path.pop() {
            return None;
        }
    }
}

/// provides the Command to run make with the given arguments in the given directory
pub fn command(args: env::Args, dir: PathBuf) -> Command {
    let mut command = Command::new("make");
    command.args(args);
    command.current_dir(dir);
    command
}
