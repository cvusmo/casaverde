// casaverde_utils/src/lib.rs
use chrono::Utc;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::fs::File;
use std::io::Write;

pub fn init_logger(
    app_name: &str,
    log_level: LevelFilter,
) -> Result<(), Box<dyn std::error::Error>> {
    let log_file_path = format!("/home/echo/projects/remote/casaverde/logs/{}.log", app_name);
    let log_file = File::create(&log_file_path)?;

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}: {}",
                Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            )
        })
        .filter(None, log_level)
        .target(Target::Pipe(Box::new(log_file)))
        .init();

    log::info!(
        "Logger initialized for {} at level {:?}",
        app_name,
        log_level
    );
    Ok(())
}
