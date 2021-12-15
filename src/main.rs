mod errors;
mod runnables;

pub use errors::UserError;
use runnables::Runnable;

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
    run(highest_runnable)
}

fn run(runnable: &Box<dyn Runnable>) -> Result<(), UserError> {
    Ok(())
}
