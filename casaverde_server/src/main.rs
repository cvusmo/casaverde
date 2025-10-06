// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server

use axum::{Router, routing::{get, post}};
use axum_server::tls_rustls::RustlsConfig;
use casaverde_server::handlers;
use casaverde_utils::dirs::get_home_dir;
use casaverde_utils::fs::read_to_string;
use casaverde_utils::io::{new_error, IoError, IoErrorKind};
use casaverde_utils::init_logger;
use log::LevelFilter;
use toml::Value;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let mut config_path = get_home_dir()
        .map_err(|e| new_error(IoErrorKind::NotFound, format!("Home directory error: {}", e)))?;
    config_path.push("casaverde/casaverde_server/config.toml");
    let config: Value = toml::from_str(&read_to_string(&config_path)?)
        .map_err(|e| new_error(IoErrorKind::Other, format!("TOML parsing error: {}", e)))?;
    let log_level = config.get("logging").and_then(|l| l.get("level")).and_then(|l| l.as_str())
        .map(|s| match s.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        })
        .unwrap_or(LevelFilter::Info);
    init_logger("casaverde_server", log_level)?;

    let addr = server_addr(&config_path)?;
    let cfg_dir = config_path.parent()
        .ok_or_else(|| new_error(IoErrorKind::Other, "Invalid config directory"))?;
    std::fs::create_dir_all(&cfg_dir)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to create config directory: {}", e)))?;

    let crt = cfg_dir.join("server.crt");
    let key = cfg_dir.join("server.key");
    if !crt.exists() || !key.exists() {
        return Err(new_error(IoErrorKind::NotFound, "Certificates missing"));
    }
    let tls = RustlsConfig::from_pem_file(&crt, &key)
        .await
        .map_err(|e| new_error(IoErrorKind::Other, format!("TLS configuration error: {}", e)))?;

    tokio::spawn(async {
        loop {
            tokio::time::sleep(Duration::from_secs(600)).await;
            casaverde_server::cache::clean_temp(Duration::from_secs(3600)).await;
        }
    });

    let app = Router::new()
        .route("/temps", get(handlers::get_temperatures))
        .route("/sensor_data", get(handlers::get_all_data).post(handlers::post_sensor_data))
        .route("/commands", get(handlers::get_commands).post(handlers::post_commands))
        .route("/configs/{controller_id}", get(handlers::get_configs))
        .route("/configs", post(handlers::post_configs));

    axum_server::bind_rustls(addr, tls)
        .serve(app.into_make_service())
        .await
        .map_err(|e| new_error(IoErrorKind::Other, format!("Server error: {}", e)))?;
    Ok(())
}

fn server_addr(config_path: &std::path::PathBuf) -> Result<std::net::SocketAddr, IoError> {
    let content = read_to_string(config_path)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to read config: {}", e)))?;
    let val: toml::Value = toml::from_str(&content)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to parse TOML: {}", e)))?;
    let ip = val.get("server")
        .and_then(|v| v.as_str())
        .ok_or_else(|| new_error(IoErrorKind::Other, "Server IP missing"))?;
    ip.parse()
        .map_err(|e| new_error(IoErrorKind::Other, format!("Invalid IP: {}", e)))
}
