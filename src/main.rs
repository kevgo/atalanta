mod errors;
mod runnables;

pub use errors::UserError;
use runnables::Runnable;
use std::env;

fn main() {
    if let Err(err) = main_with_result() {
        println!("{}", err);
    }
}

fn main_with_result() -> Result<(), UserError> {
    let runnables = runnables::find()?;
    if runnables.is_empty() {
        return Err(UserError::NoRunnableFound);
    }
    let highest_runnable = runnables.first().unwrap();
    run(highest_runnable, env::args())
}

/// executes the given Runnable with the given args
fn run(runnable: &Box<dyn Runnable>, args: env::Args) -> Result<(), UserError> {
    let mut command = runnable.command(args);
    let status = command.status().unwrap();
    if status.success() {
        Ok(())
    } else {
        Err(UserError::RunnableFailed {
            exit_code: status.code().unwrap(),
        })
    }
}
