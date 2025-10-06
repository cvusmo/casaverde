// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// lib.rs -

use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

pub fn init_logger(app_name: &str, log_level: LevelFilter) -> Result<(), std::io::Error> {
    let bin_dir = std::env::current_exe()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        .parent()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to get binary directory")
        })?
        .to_path_buf();

    // Create a logs directory in the binary folder
    let log_dir = bin_dir.join("logs");
    std::fs::create_dir_all(&log_dir)?;

    let log_file_path = log_dir.join(format!("{}.log", app_name));

    let log_file = std::fs::File::create(&log_file_path)?;

    Builder::new()
        .format(|buf, record| {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(
                buf,
                "{} [{}] - {}: {}",
                timestamp,
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            )
        })
        .filter(None, log_level)
        .target(Target::Pipe(Box::new(log_file)))
        .init();

    log::info!(
        "Logger initialized for {} at level {:?} with log file {}",
        app_name,
        log_level,
        log_file_path.display()
    );
    Ok(())
}
