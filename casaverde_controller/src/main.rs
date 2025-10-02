// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs - Entry point for casaverde_controller

use log::{info, error};
use std::fs::File;
use std::sync::Arc;
use std::time::Duration;

use env_logger::{Builder, Target};
use tokio::{spawn, sync::Mutex, time::sleep};

use casaverde_controller::client;
use casaverde_controller::config;
use casaverde_controller::controller::{Command, process_remote_readings};
use casaverde_controller::gpio;
use casaverde_controller::serial::{init_serial, send_command};
use casaverde_controller::timer::run_light_timer;

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
    let port = Arc::new(Mutex::new(init_serial(&config)?));
    match client::fetch_config(&client, &config.server, &config.controller_id).await {
        Ok(server_conf) => {
            info!("Fetched server config: {:?}", server_conf);
            config = server_conf;

            // Persist config to local config.toml
            match toml::to_string(&config) {
                Ok(toml_str) => {
                    if let Err(e) = std::fs::write("config.toml", toml_str) {
                        error!("Failed to write config.toml: {}", e);
                    } else {
                        info!("Updated local config.toml from server");
                    }
                }
                Err(e) => {
                    error!("Could not serialize new config to toml: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Could not fetch config from server: {}", e);
        }
    }

    // Spawn update task
    {
        let check_client = client.clone();
        let check_server = config.server.clone();
        let check_id = config.controller_id.clone();
        spawn(async move {
            check_config_update(check_client, check_server, check_id).await;
        });
    }

    // Spawn timer
    {
        let light_id = config.light_relay_id.clone();
        let on_hours = config.light_on_hours;
        let off_hours = config.light_off_hours;
        let timer_port = Arc::clone(&port);
        let timer_client = client.clone();
        let timer_server = config.server.clone();
        spawn(async move {
            run_light_timer(light_id, on_hours, off_hours, timer_port, timer_client, timer_server).await;
        });
    }

    // Main loop:
    loop {
        // Fetch readings
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

        // Compute remote-based commands
        let remote_commands = process_remote_readings(&readings, &config.controller_id);
        let commands: Vec<Command> = remote_commands.into_iter().collect();
        info!("Generated commands: {:?}", commands);

        // Send commands to server
        if let Err(e) = client::send_commands(&client, &config.server, &commands).await {
            error!("send_commands to server error: {}", e);
        }

        // Execute commands locally
        {
            let mut guard = port.lock().await;
            let serial_ref: &mut dyn serialport::SerialPort = &mut **guard;
            for cmd in &commands {
                info!("Executing command via serial: {:?}", cmd);
                if let Err(e) = send_command(serial_ref, cmd) {
                    error!("Error sending command via serial: {}", e);
                }
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn check_config_update(client: reqwest::Client, server: String, controller_id: String) {
    loop {
        sleep(Duration::from_secs(24 * 3600)).await;
        match client::fetch_config(&client, &server, &controller_id).await {
            Ok(new_conf) => {
                if let Ok(new_toml) = toml::to_string(&new_conf) {
                    if let Err(e) = std::fs::write("config.toml", &new_toml) {
                        error!("Failed to write config.toml: {}", e);
                    } else {
                        info!("Periodic config update: wrote new config.toml");
                    }
                } else {
                    error!("Could not serialize fetched config");
                }
            }
            Err(e) => {
                error!("Periodic fetch_config error: {}", e);
            }
        }
    }
}

