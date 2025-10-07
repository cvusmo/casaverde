// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// src/lib.rs

pub use log;

pub mod dirs;
pub mod fs;
pub mod io;
pub mod logger;
pub mod path;

pub use dirs::{get_casaverde_log_dir, get_home_dir};
pub use fs::{create_file, read_to_string, write_all};
pub use io::{new_error, IoError, IoErrorKind};
pub use logger::init_logger;
pub use path::{get_cert_path, get_config_dir, get_config_path, get_key_path, PathBuf};
