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
use toml;
use reqwest::Client;

async fn check_config_update(client: Client, server: String, controller_id: String) {
    loop {
        sleep(Duration::from_secs(24 * 3600)).await;
        match client::fetch_config(&client, &server, &controller_id).await {
            Ok(new_config) => {
                if let Ok(toml_str) = toml::to_string(&new_config) {
                    if let Err(e) = std::fs::write("config.toml", toml_str) {
                        error!("Failed to write config.toml: {}", e);
                    } else {
                        info!("Periodic config update from server");
                    }
                } else {
                    error!("Failed to serialize config to TOML");
                }
            }
            Err(e) => error!("Failed to fetch config: {}", e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = File::create("/home/echo/casaverde_controller.log")?;
    Builder::new()
        .target(Target::Pipe(Box::new(log_file)))
        .target(Target::Stdout)
        .filter(None, log::LevelFilter::Info)
        .init();

    info!("Starting casaverde_controller on {}", config::get_hostname());

    let config = config::load_config()?;
    let client = client::build_secure_client()?;
    let port = Arc::new(Mutex::new(init_serial(&config)?));
    let mut config = config::load_config()?;  // Local first

    // Fetch from server
    if let Ok(server_config) = client::fetch_config(&client, &config.server, &config.controller_id).await {
        config = server_config;
        let toml_str = toml::to_string(&config)?;
        std::fs::write("config.toml", toml_str)?;
        info!("Updated local config from server");
    }

    // Update every 24h
    let check_client = client.clone();
    let check_server = config.server.clone();
    let check_id = config.controller_id.clone();
    spawn(check_config_update(check_client, check_server, check_id));

    // Extract after update
    let light_id = config.light_relay_id.clone();
    let on_hours = config.light_on_hours;
    let off_hours = config.light_off_hours;
    let timer_port = port.clone();
    let timer_client = client.clone();
    let timer_server = config.server.clone();

    // Spawn timer
    spawn(run_light_timer(light_id, on_hours, off_hours, timer_port, timer_client, timer_server));

    loop {
        // Fetch from casaverde_server
        let readings = client::fetch_readings(&client, &config.server).await?;
        info!("Fetched readings from server: {readings:?}");
        
        // Read local temperature
        let local_temp = gpio::read_temperature();
        info!("Local temperature: {local_temp:?}");

        // Process data
        let remote_commands = process_remote_readings(&readings, &config.controller_id);
        let commands: Vec<Command> = remote_commands.into_iter().collect();
        info!("Generated commands: {commands:?}");

        client::send_commands(&client, &config.server, &commands).await?;
        for cmd in &commands {
            info!("Executing command: {cmd:?}");
            let mut port_guard = port.lock().await;
            send_command(&mut **port_guard, cmd)?;
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
