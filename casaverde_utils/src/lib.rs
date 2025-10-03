// casaverde_utils/src/lib.rs
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::fs::File;
use std::io::Write;
use tokio::time::Instant;

pub fn init_logger(app_name: &str, log_level: LevelFilter) -> Result<(), std::io::Error> {
    let log_file_path = format!(
        "/home/echo/projects/remote/casaverde/casaverde_test/{}.log",
        app_name
    );
    let log_file = File::create(&log_file_path)?;

    Builder::new()
        .format(|buf, record| {
            let timestamp = Instant::now().elapsed().as_secs();
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
        "Logger initialized for {} at level {:?}",
        app_name,
        log_level
    );
    Ok(())
}
