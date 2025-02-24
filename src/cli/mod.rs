//! This module provides functionality to interact with the shell environment from which this app is called.

mod exit_code;
mod parse;

pub use exit_code::exit_status_to_code;
pub use parse::parse;
