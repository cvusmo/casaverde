use crate::dirs::get_casaverde_log_dir;
use crate::io::{new_error, IoError, IoErrorKind};
use chrono::Local;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::{fs::File, io::Write};

/// Initializes a simple log file per binary:
///   ~/casaverde/target/build/log/<app_name>.log
pub fn init_logger(app_name: &str, level: LevelFilter) -> Result<(), IoError> {
    let log_dir = get_casaverde_log_dir()
        .map_err(|e| new_error(IoErrorKind::NotFound, format!("Log directory error: {}", e)))?;
    let log_path = log_dir.join(format!("{}.log", app_name));

    let file = File::create(&log_path).map_err(|e| {
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
