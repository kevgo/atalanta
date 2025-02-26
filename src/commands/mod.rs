//! the various sub-commands that the user can call

pub(crate) mod completions;
mod help;
mod install;
mod run;
mod setup;

pub(crate) use help::help;
pub(crate) use install::install;
pub(crate) use run::run;
pub(crate) use setup::setup;
