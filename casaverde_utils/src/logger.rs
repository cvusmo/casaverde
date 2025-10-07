// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// logger.rs

use crate::dirs::get_casaverde_log_dir;
use crate::fs::create_file;
use crate::io::{new_error, IoError, IoErrorKind};
use chrono::Local;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::fs;
use std::io::Write;

pub fn init_logger(app_name: &str, level: LevelFilter) -> Result<(), IoError> {
    let log_dir = get_casaverde_log_dir()
        .map_err(|e| new_error(IoErrorKind::NotFound, format!("Log directory error: {}", e)))?;

    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).map_err(|e| {
            new_error(
                IoErrorKind::Other,
                format!("Failed to create log directory: {}", e),
            )
        })?;
    }

    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    let log_path = log_dir.join(format!("{}_{}.log", app_name, timestamp));

    let latest_link = log_dir.join(format!("{}_latest.log", app_name));
    if latest_link.exists() {
        let _ = fs::remove_file(&latest_link);
    }
    let _ = std::os::unix::fs::symlink(&log_path, &latest_link);

    let file = create_file(&log_path).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to create log file: {}", e),
        )
    })?;

    let mut builder = Builder::new();
    builder
        .format(|buf, record| {
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(
                buf,
                "{} [{}] {}: {}",
                ts,
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            )
        })
        .filter(None, level)
        .target(Target::Pipe(Box::new(file)))
        .init();

    log::info!(
        "Logger initialized for {} at {:?}, log file: {}",
        app_name,
        level,
        log_path.display()
    );

    Ok(())
}
