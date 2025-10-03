// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs - Entry point for casaverde_controller

use casaverde_controller::{client, models};
use log::{info, error};
use std::sync::Arc;
use tokio::{spawn, sync::mpsc, time::interval};
use casaverde_controller::config;
use casaverde_controller::controller::{Command, process_remote_readings, process_local_rules, run_light_timer};
use casaverde_controller::gpio;
use casaverde_controller::serial::{init_serial, send_command, read_sensor_data};
use casaverde_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    casaverde_utils::init_logger("casaverde_controller", log::LevelFilter::Info)?;
    info!("Starting casaverde_controller on {}", config::get_hostname());

    let mut config = config::load_config()?;
    let client = client::build_secure_client()?; // Placeholder; consider hyper
    let port: Arc<tokio::sync::Mutex<Box<dyn serialport::SerialPort>>> = Arc::new(tokio::sync::Mutex::new(init_serial(&config)?));

    if let Ok(server_config) = client::fetch_config(&client, &config.server, &config.controller_id).await {
        config = server_config;
        tokio::fs::write("config.toml", toml::to_string(&config)?).await?;
        info!("Updated local config from server: {:?}", config);
    } else {
        info!("Failed to fetch config from server; using local config.toml: {:?}", config);
    }

    let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(100);

    {
        let light_id = config.light_relay_id.clone();
        let on_hours = config.light_on_hours;
        let off_hours = config.light_off_hours;
        let tx = cmd_tx.clone();
        spawn(run_light_timer(light_id, on_hours, off_hours, tx));
    }

    {
        let port = port.clone();
        let client = client.clone();
        let server = config.server.clone();
        spawn(async move {
            while let Some(cmd) = cmd_rx.recv().await {
                info!("Executing command via serial: {:?}", cmd);
                if let Err(e) = client::send_commands(&client, &server, std::slice::from_ref(&cmd)).await {
                    error!("send_commands to server error: {}", e);
                }
                let mut guard = port.lock().await;
                let port_ref: &mut dyn serialport::SerialPort = &mut **guard; // Explicit dereference
                if let Err(e) = send_command(port_ref, &cmd) {
                    error!("Error sending command via serial: {}", e);
                }
            }
        });
    }

    let mut interval = interval(tokio::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        let mut guard = port.lock().await;
        let port_ref: &mut dyn serialport::SerialPort = &mut **guard; // Explicit dereference
        let sensor_data = read_sensor_data(port_ref);
        drop(guard);

        let readings = match client::fetch_readings(&client, &config.server).await {
            Ok(r) => r,
            Err(e) => {
                error!("fetch_reads error: {}", e);
                continue;
            }
        };
        info!("Fetched readings from server: {:?}", readings);

        let local_temp = gpio::read_temperature();
        info!("Local temperature reading: {:?}", local_temp);

        let remote_commands = process_remote_readings(&readings, &config.controller_id);
        let local_commands = process_local_rules(&config, local_temp.unwrap_or_default().into());

        let mut commands = Vec::new();
        commands.extend(remote_commands);
        commands.extend(local_commands);

        for cmd in &commands {
            if let Err(e) = cmd_tx.send(cmd.clone()).await {
                error!("Failed to enqueue command: {}", e);
            }
        }

        if let Some(mut devices) = sensor_data {
            devices.push(models::DeviceReading {
                id: "blackbeard-cpu".to_string(),
                value: local_temp,
            });
            let reading = models::SensorReading {
                client_id: config.controller_id.clone(),
                devices,
            };
            if let Err(e) = client::send_sensor_data(&client, &config.server, &reading).await {
                error!("Failed to send sensor data: {}", e);
            } else {
                info!("Sent sensor data to server: {:?}", reading);
            }
        }
    }
}
