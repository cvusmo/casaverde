// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/client.rs - HTTP client setup and data fetching

use reqwest::{Client, Certificate};
use std::fs;
use log::{error, info};

use crate::controller::CachedData;

pub fn build_secure_client() -> Result<Client, Box<dyn std::error::Error>> {
    let cert_data = fs::read("server.crt").map_err(|e| {
        error!("Failed to read server.crt: {e}");
        e
    })?;
    let cert = Certificate::from_pem(&cert_data).map_err(|e| {
        error!("Invalid certificate: {e}");
        e
    })?;
    info!("Certificate loaded successfully");

    Ok(Client::builder()
        .add_root_certificate(cert)
        .use_rustls_tls()
        .min_tls_version(reqwest::tls::Version::TLS_1_3)
        .build()
        .map_err(|e| {
            error!("Failed to build secure client: {e}");
            e
        })?)
}

pub async fn fetch_readings(client: &Client, server: &str) -> Result<Vec<CachedData>, Box<dyn std::error::Error>> {
    let url = format!("{server}/temps");
    let resp = client.get(&url).send().await?.json::<Vec<CachedData>>().await?;
    info!("Fetched readings from {url}");
    Ok(resp)
}
