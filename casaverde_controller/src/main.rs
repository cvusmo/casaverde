// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs - Entry point for casaverde_controller

use casaverde_controller::models;
use log::{info, error};
use std::fs::File;
use std::sync::Arc;
use std::time::Duration;
use env_logger::{Builder, Target};
use tokio::{spawn, sync::{mpsc, Mutex}, time::sleep};

use casaverde_controller::client;
use casaverde_controller::config;
use casaverde_controller::controller::{Command, process_remote_readings, process_local_rules, run_light_timer};
use casaverde_controller::gpio;
use casaverde_controller::serial::{init_serial, send_command, read_sensor_data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = File::create("/home/echo/casaverde_controller.log")?;
    Builder::new()
        .target(Target::Pipe(Box::new(log_file)))
        .target(Target::Stdout)
        .filter(None, log::LevelFilter::Info)
        .init();

    info!("Starting casaverde_controller on {}", config::get_hostname());

    let mut config = config::load_config()?;
    let client = client::build_secure_client()?;
    let port: Arc<Mutex<Box<dyn serialport::SerialPort>>> = Arc::new(Mutex::new(init_serial(&config)?));

    // Fetch and update config from server
    if let Ok(server_config) = client::fetch_config(&client, &config.server, &config.controller_id).await {
        config = server_config;
        let toml_str = toml::to_string(&config)?;
        std::fs::write("config.toml", toml_str)?;
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
        spawn(async move {
            run_light_timer(light_id, on_hours, off_hours, tx).await;
        });
    }

    {
        let port = Arc::clone(&port);
        let client = client.clone();
        let server = config.server.clone();
        spawn(async move {
            while let Some(cmd) = cmd_rx.recv().await {
                info!("Executing command via serial: {:?}", cmd);
                if let Err(e) = client::send_commands(&client, &server, &[cmd.clone()]).await {
                    error!("send_commands to server error: {}", e);
                }
                let mut guard = port.lock().await;
                let serial_ref: &mut dyn serialport::SerialPort = &mut **guard;
                if let Err(e) = send_command(serial_ref, &cmd) {
                    error!("Error sending command via serial: {}", e);
                }
            }
        });
    }

loop {
    let mut guard = port.lock().await;
    let serial_ref: &mut dyn serialport::SerialPort = &mut **guard;
    let sensor_data = read_sensor_data(serial_ref);
    drop(guard); 

    let readings = match client::fetch_readings(&client, &config.server).await {
        Ok(r) => r,
        Err(e) => {
            error!("fetch_readings error: {}", e);
            sleep(Duration::from_secs(5)).await;
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

    for cmd in commands {
        if let Err(e) = cmd_tx.send(cmd).await {
            error!("Failed to enqueue command: {}", e);
        }
    }

    // Always create and send sensor reading, including local CPU temp
    let mut devices = sensor_data.unwrap_or_default();
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

    sleep(Duration::from_secs(5)).await;
    }
}
