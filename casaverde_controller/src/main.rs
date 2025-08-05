// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs - Entry point for casaverde_controller

use log::info;
use std::time::Duration;

use casaverde_controller::config;
use casaverde_controller::client;
use casaverde_controller::controller::{Command, process_remote_readings, process_local_readings};
use casaverde_controller::gpio;
use casaverde_controller::serial::{init_serial, send_command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting casaverde_controller on {}", config::get_hostname());

    let config = config::load_config()?;
    let client = client::build_secure_client()?;
    let mut port = init_serial(&config)?;

    loop {
        // Fetch remote readings from casaverde_server
        let readings = client::fetch_readings(&client, &config.server).await?;
        info!("Fetched readings from server: {:?}", readings);
        
        // Read local temperature from DS18B20
        let local_temp = gpio::read_temperature();
        info!("Local temperature: {:?}", local_temp);

        // Process both remote and local data
        let remote_commands = process_remote_readings(&readings, &config.controller_id);
        let local_commands = process_local_readings(local_temp, &config.controller_id);
        let commands: Vec<Command> = remote_commands.into_iter().chain(local_commands).collect();
        info!("Generated commands: {:?}", commands);

        // Send commands to casaverde_server (removed extra controller_id argument)
        client::send_commands(&client, &config.server, &commands).await?;
        for cmd in &commands {
            send_command(&mut *port, cmd)?; // Removed .clone(), passing reference
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
