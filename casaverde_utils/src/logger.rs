// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// logger.rs -

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }
}

#[derive(Debug)]
pub enum LoggerError {
    IoError(io::Error),
    InvalidBinaryPath,
}

impl From<io::Error> for LoggerError {
    fn from(err: io::Error) -> Self {
        LoggerError::IoError(err)
    }
}

impl std::fmt::Display for LoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoggerError::IoError(err) => write!(f, "IO error: {}", err),
            LoggerError::InvalidBinaryPath => write!(f, "Failed to get binary directory"),
        }
    }
}

impl std::error::Error for LoggerError {}

pub struct CustomLogger {
    file: Arc<Mutex<File>>,
    level: LogLevel,
}

impl CustomLogger {
    fn new(file: File, level: LogLevel) -> Self {
        CustomLogger {
            file: Arc::new(Mutex::new(file)),
            level,
        }
    }

    pub fn log(&self, level: LogLevel, module: &str, message: &str) -> Result<(), LoggerError> {
        if level > self.level {
            return Ok(());
        }

        let mut file = self
            .file
            .lock()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Mutex lock failed: {}", e)))?;

        let timestamp = format_utc_time();
        writeln!(
            file,
            "{} [{}] {}: {}",
            timestamp,
            level.as_str(),
            module,
            message
        )?;
        file.flush()?;
        Ok(())
    }
}

pub fn init_logger(app_name: &str, log_level: LogLevel) -> Result<Arc<CustomLogger>, LoggerError> {
    let bin_dir = std::env::current_exe()
        .map_err(|e| LoggerError::IoError(e))?
        .parent()
        .ok_or(LoggerError::InvalidBinaryPath)?
        .to_path_buf();

    let log_dir = bin_dir.join("logs");
    fs::create_dir_all(&log_dir)?;

    let log_path = log_dir.join(format!("{}.log", app_name));
    let log_file = File::create(&log_path)?;

    let logger = Arc::new(CustomLogger::new(log_file, log_level));

    // Synchronous initialization log (std-only, no async)
    logger.log(
        LogLevel::Info,
        "casaverde_utils::logger",
        &format!(
            "Custom logger initialized for {} at level {:?} -> {}",
            app_name,
            log_level,
            log_path.display()
        ),
    )?;

    Ok(logger)
}

// Std-only UTC timestamp formatting (no external crates)
fn format_utc_time() -> String {
    let now = SystemTime::now();
    let secs = now.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs()) as i64;

    let mut seconds_in_day = secs % 86400;
    if seconds_in_day < 0 {
        seconds_in_day += 86400;
    }
    let mut days = secs / 86400;

    let hour = (seconds_in_day / 3600) as u32;
    seconds_in_day %= 3600;
    let minute = (seconds_in_day / 60) as u32;
    seconds_in_day %= 60;
    let second = seconds_in_day as u32;

    let mut year: i64 = 1970;
    while days >= days_in_year(year) {
        days -= days_in_year(year);
        year += 1;
    }
    if days < 0 {
        year -= 1;
        days += days_in_year(year);
    }

    let mut month: u32 = 1;
    while days >= days_in_month(year, month) as i64 {
        days -= days_in_month(year, month) as i64;
        month += 1;
    }

    let day = (days + 1) as u32;

    format!("{:
