use super::Runnable;
use crate::UserError;
use std::env;
use std::path::PathBuf;

/// detects a Makefile in the current or a parent directory
pub fn detect() -> Result<Box<dyn Runnable>, UserError> {
    let mut dir = env::current_dir().expect("Cannot get current directory");
    Ok(Box::new(MakefileRunnable { dir }))
}

pub struct MakefileRunnable {
    /// path of the Makefile
    dir: PathBuf,
}

impl Runnable for MakefileRunnable {
    fn run(&self) {
        //
    }
}
