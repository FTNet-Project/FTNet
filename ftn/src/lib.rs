// #![deny(unused_extern_crates)]
// #![deny(unused_crate_dependencies)]
#![deny(unsafe_code)]

extern crate self as ftn;

mod cli;
pub mod dotftn;
mod identity;
mod start;

pub use cli::{Cli, Command};
pub use start::start;
