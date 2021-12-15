use crate::UserError;
use std::env;
use std::process::Command;

mod makefile;

/// describes a runnable thing
pub trait Runnable {
    /// provides the command to run
    fn command(&self, args: env::Args) -> Command;
}

pub fn find() -> Result<Vec<Box<dyn Runnable>>, UserError> {
    let mut result: Vec<Box<dyn Runnable>> = Vec::new();
    if let Some(runnable) = makefile::detect() {
        result.push(runnable);
    }
    Ok(result)
}
