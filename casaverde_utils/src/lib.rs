// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// src/lib.rs

pub mod dirs;
pub mod fs;
pub mod io;
pub mod log;
pub mod path;
pub mod time;
pub mod uuid;

pub use dirs::{get_casaverde_log_dir, get_home_dir};
pub use fs::{create_file, read_to_string, write_all};
pub use io::{new_error, IoError, IoErrorKind};
pub use log::{debug, error, info, init_logger, log, trace, warn};
pub use path::{get_cert_path, get_config_dir, get_config_path, get_key_path, PathBuf};
pub use time::{current_timestamp, delay, formatted_timestamp, init_time};
pub use uuid::generate_id;

// Re-exports from casaverde_log
pub use casaverde_log::level::{Level, LevelFilter};
pub use casaverde_log::logger::Logger;
pub use casaverde_log::timestamp::unix_time_to_human_readable;

// Re-exports from casaverde_time
pub use casaverde_time::formatted;
pub use casaverde_time::timer;
pub use casaverde_time::timestamp;

/// Returns the version of the casaverde_utils crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
