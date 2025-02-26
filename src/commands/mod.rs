//! the various sub-commands that the user can call

pub mod completions;
mod help;
mod run;
mod setup;

pub(crate) use help::help;
pub use run::run;
pub use setup::setup;
