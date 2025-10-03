// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/client.rs - HTTP client setup and data fetching

use crate::config;
use crate::config::Config;
use crate::controller::Command;
use crate::models::ConfigEntry;
use log::{error, info};
use reqwest::{Client, Certificate};
use serde::Serialize;
use serde_json::Value;
use std::fs;

#[derive(Serialize, Debug)]
pub struct CommandPayload {
    pub controller_id: String,
    pub commands: Vec<CommandData>,
}

#[derive(Serialize, Debug)]
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

pub async fn fetch_readings(client: &Client, server: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("{server}/temps");
    let resp = client.get(&url).send().await?.json::<Value>().await?;
    info!("Fetched readings from {url}");
    Ok(resp)
}

pub async fn fetch_config(client: &Client, server: &str, controller_id: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let url = format!("{server}/configs/{controller_id}");
    let resp = client.get(&url).send().await?.json::<ConfigEntry>().await?;
    Ok(Config {
        server: resp.current.server,
        controller_id: resp.current.controller_id,
        serial_port: resp.current.serial_port,
        light_relay_id: resp.current.light_relay_id,
        light_on_hours: resp.current.light_on_hours,
        light_off_hours: resp.current.light_off_hours,
    })
}

pub async fn simulation_commands(client: &Client, server: &str, is_simulation: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_simulation {
        let commands = vec![
            CommandData { action: "GET".to_string(), device_id: "blackbeard-cpu".to_string() },
            CommandData { action: "GET".to_string(), device_id: "solar-1".to_string() },
            CommandData { action: "GET".to_string(), device_id: "moisture-1".to_string() },
            CommandData { action: "GET".to_string(), device_id: "humidity-1".to_string() },
            CommandData { action: "GET".to_string(), device_id: "water-1".to_string() },
            CommandData { action: "GET".to_string(), device_id: "relay-1".to_string() },
        ];
        let payload = CommandPayload {
            controller_id: config::get_hostname(),
            commands,
        };
        let url = format!("{server}/commands");
        let resp = client.post(&url).json(&payload).send().await?;
        if resp.status().is_success() {
            info!("Successfully sent simulation commands to {url}: {:?}", payload);
            Ok(())
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            error!("Failed to send simulation commands to {url}: status {status}, response {text}");
            Err(format!("Failed to send commands: status {status}").into())
        }
    } else {
        Ok(())
    }
}

pub async fn send_commands(client: &Client, server: &str, commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{server}/commands");
    let payload = CommandPayload {
        controller_id: config::get_hostname(),
        commands: commands.iter().map(|cmd| match cmd {
            Command::TurnOnCooling => CommandData { action: "ON".to_string(), device_id: "FAN1".to_string() },
            Command::TurnOffCooling => CommandData { action: "OFF".to_string(), device_id: "FAN1".to_string() },
            Command::TurnOnMoisture => CommandData { action: "ON".to_string(), device_id: "moisture-1".to_string() },
            Command::TurnOffMoisture => CommandData { action: "OFF".to_string(), device_id: "moisture-1".to_string() },
            Command::OpenValve => CommandData { action: "OPEN".to_string(), device_id: "water-1".to_string() },
            Command::CloseValve => CommandData { action: "CLOSE".to_string(), device_id: "water-1".to_string() },
            Command::TurnOnSolar => CommandData { action: "ON".to_string(), device_id: "solar-1".to_string() },
            Command::TurnOffSolar => CommandData { action: "OFF".to_string(), device_id: "solar-1".to_string() },
            Command::TurnOnHumidity => CommandData { action: "ON".to_string(), device_id: "humidity-1".to_string() },
            Command::TurnOffHumidity => CommandData { action: "OFF".to_string(), device_id: "humidity-1".to_string() },
            Command::SetPWM(pwm) => CommandData { action: "SET".to_string(), device_id: format!("FAN1_{pwm}") },
        }).collect(),
    };

    let resp = client.post(&url).json(&payload).send().await?;
    if resp.status().is_success() {
        info!("Successfully sent commands to {url}: {:?}", payload);
        return Ok(());
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        error!("Failed to send commands to {url}: status {status}, response {text}");
        return Err(format!("Failed to send commands: status {status}").into());
    }
}

pub async fn send_sensor_data(
    client: &reqwest::Client,
    server: &str,
    reading: &crate::models::SensorReading,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/sensor_data", server);
    client.post(&url).json(reading).send().await?.error_for_status()?;
    Ok(())
}
