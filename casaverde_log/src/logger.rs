// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_log
// src/logger.rs

use super::level::{Level, LevelFilter};
use super::timestamp::unix_time_to_human_readable;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Logger {
    file: Arc<Mutex<File>>,
    level: LevelFilter,
}

impl Logger {
    pub fn new(
        app_name: &str,
        level: LevelFilter,
        log_dir: PathBuf,
    ) -> Result<Self, std::io::Error> {
        if !log_dir.exists() {
            fs::create_dir_all(&log_dir)?;
        }

        let log_path = log_dir.join(format!("{}.log", app_name));

        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        let file = Arc::new(Mutex::new(file));

        let ts = unix_time_to_human_readable(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );

        let msg = format!(
            "Logger initialized for {} at {:?}, log file: {}",
            app_name,
            level,
            log_path.display()
        );

        let line = format!("{} [INFO] {}\n", ts, msg);

        file.lock().unwrap().write_all(line.as_bytes())?;

        Ok(Self { file, level })
    }

    pub fn log(&mut self, log_level: Level, msg: &str) -> Result<(), std::io::Error> {
        if (self.level as u32) < (log_level as u32) {
            return Ok(());
        }

        let ts = unix_time_to_human_readable(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );

        let level_str = match log_level {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        };

        let line = format!("{} [{}] {}\n", ts, level_str, msg);

        self.file.lock().unwrap().write_all(line.as_bytes())
    }

    pub fn error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(Level::Error, msg)
    }

    pub fn warn(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(Level::Warn, msg)
    }

    pub fn info(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(Level::Info, msg)
    }

    pub fn debug(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(Level::Debug, msg)
    }

    pub fn trace(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.log(Level::Trace, msg)
    }
}
