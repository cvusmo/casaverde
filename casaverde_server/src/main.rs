// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/main.rs

use axum::{
    Router,
    routing::{get, post},
};
use axum_server::tls_rustls::RustlsConfig;
use dirs::config_dir;
use log::info;
use std::net::SocketAddr;
use std::{fs, io};
use toml::Value;
use casaverde_server::handlers;
use casaverde_utils;

#[tokio::main]
async fn main() -> io::Result<()> {
    casaverde_utils::init_logger("casaverde_server", log::LevelFilter::Info)?;
    info!("Starting casaverde_server");

    let addr = get_server_addr()?;

    let mut config_dir = config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    config_dir.push("casaverde_server");

    fs::create_dir_all(&config_dir)?;

    let mut cert_path = config_dir.clone();
    cert_path.push("server.crt");
    let mut key_path = config_dir.clone();
    key_path.push("server.key");

    if !cert_path.exists() || !key_path.exists() {
        eprintln!("Error: Certificates not found in {}", config_dir.display());
        eprintln!("Generate them by running './build.sh' in the project directory");
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Certificate or key file missing",
        ));
    }

    let config = RustlsConfig::from_pem_file(&cert_path, &key_path).await?;
    info!("Server running on https://{addr}");

    let app = Router::new()
        .route("/temps", get(handlers::get_temperatures))
        .route(
            "/sensor_data",
            get(handlers::get_all_data).post(handlers::post_sensor_data),
        )
        .route(
            "/commands",
            get(handlers::get_commands).post(handlers::post_commands),
        )
        .route("/configs/{controller_id}", get(handlers::get_configs))
        .route("/configs", post(handlers::post_configs));

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn get_server_addr() -> io::Result<SocketAddr> {
    let mut config_path = config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    config_path.push("casaverde_server/config.toml");

    let config_str = fs::read_to_string(&config_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read config.toml: {}", e),
        )
    })?;

    let config: Value = toml::from_str(&config_str).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to parse config.toml: {}", e),
        )
    })?;

    let server_ip = config
        .get("server")
        .and_then(|v| v.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Server address not found in config.toml"))?;

    server_ip.parse().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Invalid server address format in config.toml: {}", e),
        )
    })
}
