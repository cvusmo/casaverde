// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// src/log.rs

use crate::dirs::get_casaverde_log_dir;
use crate::io::{new_error, IoError, IoErrorKind};
use casaverde_log::level::{Level, LevelFilter};
use casaverde_log::logger::Logger;

/// Initializes a logger for the given application name and log level.
/// Logs are written to ~/casaverde/build_output/linux/logs/<app_name>.log.
pub fn init_logger(app_name: &str, level: LevelFilter) -> Result<Logger, IoError> {
    let log_dir = get_casaverde_log_dir()
        .map_err(|e| new_error(IoErrorKind::NotFound, format!("Log directory error: {}", e)))?;
    Logger::new(app_name, level, log_dir).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to initialize logger: {}", e),
        )
    })
}

/// Logs a message at the specified level using the provided logger.
pub fn log(logger: &mut Logger, level: Level, message: &str) -> Result<(), IoError> {
    logger
        .log(level, message)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to write log: {}", e)))
}

/// Logs an error message.
pub fn error(logger: &mut Logger, message: &str) -> Result<(), IoError> {
    logger.error(message).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to write error log: {}", e),
        )
    })
}

/// Logs a warning message.
pub fn warn(logger: &mut Logger, message: &str) -> Result<(), IoError> {
    logger.warn(message).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to write warn log: {}", e),
        )
    })
}

/// Logs an info message.
pub fn info(logger: &mut Logger, message: &str) -> Result<(), IoError> {
    logger.info(message).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to write info log: {}", e),
        )
    })
}

/// Logs a debug message.
pub fn debug(logger: &mut Logger, message: &str) -> Result<(), IoError> {
    logger.debug(message).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to write debug log: {}", e),
        )
    })
}

/// Logs a trace message.
pub fn trace(logger: &mut Logger, message: &str) -> Result<(), IoError> {
    logger.trace(message).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to write trace log: {}", e),
        )
    })
}
