// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/main.rs

use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{fs, io};
use std::net::SocketAddr;
use dirs::config_dir;

use casaverde_server::handlers;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = get_server_addr();

    // Construct path to cert and key
    let mut config_dir = config_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    config_dir.push("casaverde_server");

    // Create config directory if it doesn't exist
    fs::create_dir_all(&config_dir)?;

    let mut cert_path = config_dir.clone();
    cert_path.push("server.crt");
    let mut key_path = config_dir.clone();
    key_path.push("server.key");

    // Check if certificates exist
    if !cert_path.exists() || !key_path.exists() {
        eprintln!("Error: Certificates not found in {}", config_dir.display());
        eprintln!("Generate them by running './setup.sh' in the project directory");
        return Err(io::Error::new(io::ErrorKind::NotFound, "Certificate or key file missing"));
    }

    // Load TLS configuration
    let config = RustlsConfig::from_pem_file(&cert_path, &key_path).await?;
    println!("Server running on https://{addr}");

    let app = Router::new()
        .route("/temps", get(handlers::get_temperatures))
        .route("/sensor_data", get(handlers::get_all_data).post(handlers::post_sensor_data));

    // Serve with TLS
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn get_server_addr() -> SocketAddr {
    std::env::var("SERVER_IP")
        .unwrap_or("10.0.0.12:3000".to_string())
        .parse()
        .expect("Invalid SERVER_IP format")
}
