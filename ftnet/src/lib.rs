// #![deny(unused_extern_crates)]
// #![deny(unused_crate_dependencies)]
#![deny(unsafe_code)]

extern crate self as ftnet;

mod cli;

pub use cli::Cli;
