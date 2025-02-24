//! the various sub-commands that the user can call

pub mod completions;
pub mod list;
mod run;
mod setup;

pub use list::list;
pub use run::run;
pub use setup::setup;
