// #![deny(unused_extern_crates)]
// #![deny(unused_crate_dependencies)]
#![deny(unsafe_code)]

extern crate self as skynet;

mod expose_http;

pub use expose_http::expose_http;
