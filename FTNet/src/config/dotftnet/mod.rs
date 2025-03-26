//! The FTNet folder
//!
//! The location of this folder is platform-specific, on Linux it is either
//! $HOME/.local/share/FTNet or $XDG_DATA_HOME/FTNet, on MacOS it is $HOME/Library/Application
//! Support/com.FifthTry.FTNet and on Windows: {FOLDERID_RoamingAppData}\FTNet\data which is usually
//! C:\Users\Alice\AppData\Roaming\FifthTry\FTNet\data.
//!
//! The folder contains a lock file, `$FTNet/FTNet.lock, which is used to ensure only one instance
//! of `FTNet` is running.
//!
//! The folder contains more folders like `identities`, `logs` and maybe `config.json` etc. in
//! the future.
//!
//! The identities folder is the most interesting one, it contains one folder for every identity
//! that exists on this machine. The content of single `identity` folder is described
//! in `identity/create.rs`.

pub const LOCK_FILE: &str = "FTNet.lock";

mod init_if_required;
mod lock;

pub use init_if_required::init_if_required;
pub use lock::{exclusive, lock_file};
