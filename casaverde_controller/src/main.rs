// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs - Entry point for casaverde_controller

use casaverde_controller::timer::run_light_timer;
use casaverde_controller::{client, models};
use casaverde_utils::log::{error, info};
use casaverde_utils::init_logger;
use casaverde_utils::log::LevelFilter;
use std::sync::Arc;
use tokio::{spawn, sync::mpsc, time::interval};
use casaverde_controller::config;
use casaverde_controller::controller::{process_local_rules, process_remote_readings, Command};
use casaverde_controller::gpio;
use casaverde_controller::serial::{init_serial, send_serial_command};
use casaverde_controller::sensors::SensorController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    init_logger("casaverde_controller", LevelFilter::Info)?;
    info!("Starting casaverde_controller on {}", config::get_hostname());

    // Load local config
    let mut config = config::load_config()?;
    let client = client::build_secure_client()?;
    let port: Arc<tokio::sync::Mutex<Box<dyn serialport::SerialPort>>> =
        Arc::new(tokio::sync::Mutex::new(init_serial(&config.serial_port.clone().unwrap_or_default())?));

    // Fetch updated config from server
    if let Ok(server_config) = client::fetch_config(&client, &config.server, &config.controller_id).await {
        config = server_config;
        tokio::fs::write("config.toml", toml::to_string(&config)?).await?;
        info!("Updated local config from server: {:?}", config);
    } else {
        info!("Failed to fetch config from server; using local config.toml: {:?}", config);
    }

    let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(100);

    // Light timer
    {
        let on_hours = config.light_on_hours;
        let off_hours = config.light_off_hours;
        let tx = cmd_tx.clone();
        spawn(run_light_timer(String::new(), on_hours, off_hours, tx));
    }

    // Command executor
    {
        let port = port.clone();
        let client = client.clone();
        let server = config.server.clone();
        let is_simulation = std::env::var("SIMULATION_MODE").map(|v| v == "1").unwrap_or(false);

        spawn(async move {
            while let Some(cmd) = cmd_rx.recv().await {
                info!("Executing command: {:?}", cmd);

                // Send command to server
                if let Err(e) = client::send_commands(&client, &server, std::slice::from_ref(&cmd)).await {
                    error!("send_commands to server error: {}", e);
                }

                // Send command via serial port
                let mut guard = port.lock().await;
                let port_ref: &mut dyn serialport::SerialPort = &mut **guard;
                if let Err(e) = send_serial_command(port_ref, &cmd) {
                    error!("Error sending command via serial: {}", e);
                }

                // Simulation mode
                if is_simulation {
                    if let Err(e) = client::simulation_commands(&client, &server, is_simulation).await {
                        error!("Simulation command error: {}", e);
                    }
                }
            }
        });
    }

    // Main sensor & control loop
    let mut interval = interval(tokio::time::Duration::from_secs(5));
    loop {
        interval.tick().await;

        // Process sensors
        let readings: Vec<models::DeviceReading> = {
            let mut guard = port.lock().await;
            let port_ref: &mut dyn serialport::SerialPort = &mut **guard;
            let mut sensor_controller = SensorController::new(port_ref);
            sensor_controller.process_sensors()
        };

        // Add CPU temperature reading
        let cpu_temp = gpio::read_temperature();
        let mut full_readings = readings.clone();
        full_readings.push(models::DeviceReading {
            id: "blackbeard-cpu".to_string(),
            value: cpu_temp,
        });

        // Send sensor data to server
        let sensor_payload = models::SensorReading {
            client_id: config.controller_id.clone(),
            devices: full_readings.clone(),
        };
        if let Err(e) = client::send_sensor_data(&client, &config.server, &sensor_payload).await {
            error!("Failed to send sensor data: {}", e);
        } else {
            info!("Sent sensor data to server: {:?}", sensor_payload);
        }

        // Fetch remote readings
        let remote_readings = match client::fetch_readings(&client, &config.server).await {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to fetch remote readings: {}", e);
                continue;
            }
        };

        // Compute commands from remote readings & local rules
        let remote_commands = process_remote_readings(&remote_readings, &config.controller_id);
        let local_commands = process_local_rules(&config, cpu_temp.unwrap_or_default().into());

        // Enqueue all commands
        for cmd in remote_commands.into_iter().chain(local_commands.into_iter()) {
            if let Err(e) = cmd_tx.send(cmd).await {
                error!("Failed to enqueue command: {}", e);
            }
        }
    }
}
