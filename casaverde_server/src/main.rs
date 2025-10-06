// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/main.rs

use axum::{Router, routing::{get, post}};
use axum_server::tls_rustls::RustlsConfig;
use casaverde_server::handlers;
use casaverde_utils::dirs::get_home_dir;
use casaverde_utils::io::{new_error, IoError, IoErrorKind};
use casaverde_utils::init_logger;
use log::LevelFilter;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    init_logger("casaverde_server", LevelFilter::Info)?;
    let addr = server_addr()?;

    let mut cfg_dir = get_home_dir()
        .map_err(|e| new_error(IoErrorKind::NotFound, format!("Config directory error: {}", e)))?;
    cfg_dir.push("casaverde/casaverde_server");
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

fn server_addr() -> Result<std::net::SocketAddr, IoError> {
    let mut cfg = get_home_dir()
        .map_err(|e| new_error(IoErrorKind::NotFound, format!("Config directory error: {}", e)))?;
    cfg.push("casaverde/casaverde_server/config.toml");

    let content = std::fs::read_to_string(&cfg)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to read config: {}", e)))?;

    let val: toml::Value = toml::from_str(&content)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to parse TOML: {}", e)))?;

    let ip = val.get("server")
        .and_then(|v| v.as_str())
        .ok_or_else(|| new_error(IoErrorKind::Other, "Server IP missing"))?;

    ip.parse()
        .map_err(|e| new_error(IoErrorKind::Other, format!("Invalid IP: {}", e)))
}
