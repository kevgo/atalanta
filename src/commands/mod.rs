//! the various sub-commands that the user can call

pub mod completions;
mod run;
mod setup;

pub use run::run;
pub use setup::setup;
