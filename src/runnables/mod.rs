use crate::UserError;

mod makefile;

/// describes a runnable thing
pub trait Runnable {
    /// runs this runnable
    fn run(&self);
}

pub fn find() -> Result<Vec<Box<dyn Runnable>>, UserError> {
    let result: Vec<Box<dyn Runnable>> = Vec::new();
    Ok(result)
}
