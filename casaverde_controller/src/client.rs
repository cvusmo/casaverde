// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/client.rs - HTTP client setup and data fetching

use crate::config;
use crate::controller::{Command, CachedData};
use log::{error, info};
use reqwest::{Client, Certificate};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct CommandPayload {
    pub controller_id: String,
    pub commands: Vec<CommandData>,
}

#[derive(Serialize)]
pub struct CommandData {
    pub action: String,
    pub device_id: String,
}

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
        .danger_accept_invalid_certs(true)
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

pub async fn send_commands(client: &Client, server: &str, commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{server}/commands");
    let payload = CommandPayload {
        controller_id: config::get_hostname(),
        commands: commands.iter().map(|cmd| match cmd {
            Command::TurnOnCooling(id) => CommandData {
                action: "TurnOnCooling".to_string(),
                device_id: id.clone(),
            },
            Command::TurnOffCooling(id) => CommandData {
                action: "TurnOffCooling".to_string(),
                device_id: id.clone(),
            },
        }).collect(),
    };

    let resp = client.post(&url).json(&payload).send().await?;
    if resp.status().is_success() {
        info!("Successfully sent commands to {url}: {:?}", payload);
        Ok(())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        error!("Failed to send commands to {url}: status {status}, response {text}");
        Err(format!("Failed to send commands: status {status}").into())
    }
}
