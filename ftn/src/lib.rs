#![deny(unused_extern_crates)]
#![deny(unused_crate_dependencies)]
#![deny(unsafe_code)]

extern crate self as ftn;

mod cli;
mod config;
mod identity;
mod start;
pub mod utils;

pub use cli::{Cli, Command};
pub use config::Config;
pub use identity::Identity;
pub use start::start;
