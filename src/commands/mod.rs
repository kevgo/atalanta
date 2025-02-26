//! the various sub-commands that the user can call

pub(crate) mod completions;
mod run;
mod setup;

pub(crate) use run::run;
pub(crate) use setup::setup;
