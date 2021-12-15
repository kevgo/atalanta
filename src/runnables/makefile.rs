use super::Runnable;
use std::env;
use std::path::PathBuf;

/// detects a Makefile in the current or a parent directory
pub fn detect() -> Option<Box<dyn Runnable>> {
    let dir = env::current_dir().expect("Cannot get current directory");
    loop {
        let mut path = dir.join("Makefile");
        if path.is_file() {
            return Some(Box::new(MakefileRunnable { path }));
        }
        if !path.pop() || !path.pop() {
            return None;
        }
    }
}

pub struct MakefileRunnable {
    /// path of the Makefile
    path: PathBuf,
}

impl Runnable for MakefileRunnable {
    fn run(&self) {
        //
    }
}
