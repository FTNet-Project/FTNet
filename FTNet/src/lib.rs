#![deny(unused_extern_crates)]
#![deny(unused_crate_dependencies)]
#![deny(unsafe_code)]

// TODO: Remove this and separate ftnet binary from library to get rid of unused_crate_dependencies
// lint check errors
// Only the binary is using the following crates:
use fastn_observer as _;
use tracing_subscriber as _;
use directories as _;

extern crate self as ftnet;

mod cli;
mod client;
mod config;
pub mod control_server;
mod counters;
pub mod http;
mod identity;
mod peer_server;
mod protocol;
mod start;
pub mod utils;

pub use cli::{Cli, Command};
pub use config::Config;
pub use counters::{
    CONTROL_CONNECTION_COUNT, CONTROL_REQUEST_COUNT, IN_FLIGHT_REQUESTS,
    OPEN_CONTROL_CONNECTION_COUNT,
};
pub use identity::{Identity, PeerIdentity};
pub use protocol::Protocol;
pub use start::start;

/// Iroh supports multiple protocols, and we do need multiple protocols, lets say one for proxying
/// TCP connection, another for proxying HTTP connection, and so on. But if we use different APNS
/// to handle them, we will end up creating more connections than minimally required (one connection
/// can only talk one APNS). So, we use a single APNS for all the protocols, and we use the first
/// line of the input to determine the protocol.
const APNS_IDENTITY: &[u8] = b"/FTNet/identity/0.1";
