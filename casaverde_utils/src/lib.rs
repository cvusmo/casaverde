// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// lib.rs

pub use log;

pub mod dirs;
pub mod io;
pub mod logger;

pub use dirs::{get_casaverde_log_dir, get_home_dir};
pub use io::{new_error, IoError, IoErrorKind};
pub use logger::init_logger;
