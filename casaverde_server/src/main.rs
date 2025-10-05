// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/main.rs

use axum::{Router, routing::{get, post}};
use axum_server::tls_rustls::RustlsConfig;
use casaverde_server::handlers;
use casaverde_utils;
use std::{fs, io, net::SocketAddr};
use dirs::config_dir;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    casaverde_utils::init_logger("casaverde_server", log::LevelFilter::Info)?;
    let addr = server_addr()?;

    let mut cfg_dir = config_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    cfg_dir.push("casaverde_server");
    fs::create_dir_all(&cfg_dir)?;

    let crt = cfg_dir.join("server.crt");
    let key = cfg_dir.join("server.key");
    if !crt.exists() || !key.exists() { return Err(io::Error::new(io::ErrorKind::NotFound, "Certificates missing")) }
    let tls = RustlsConfig::from_pem_file(&crt, &key).await?;

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

    axum_server::bind_rustls(addr, tls).serve(app.into_make_service()).await?;
    Ok(())
}

fn server_addr() -> io::Result<SocketAddr> {
    let mut cfg = config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config dir missing"))?;
    cfg.push("casaverde_server/config.toml");

    let content = fs::read_to_string(&cfg)?;

    let val: toml::Value = toml::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to parse TOML: {}", e)))?;

    let ip = val.get("server")
        .and_then(|v| v.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Server IP missing"))?;

    ip.parse().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Invalid IP: {}", e)))
}
