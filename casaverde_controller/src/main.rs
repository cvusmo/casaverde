// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs - Entry point for casaverde_controller

use log::info;
use std::time::Duration;

use casaverde_controller::config;
use casaverde_controller::client;
use casaverde_controller::controller;
use casaverde_controller::gpio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting casaverde_controller on {}", config::get_hostname());

    let config = config::load_config()?;
    let client = client::build_secure_client()?;
    gpio::initialize_gpio(); // Initialize GPIO for 74HC595

    loop {
        // Fetch remote readings
        let readings = client::fetch_readings(&client, &config.server).await?;
        
        // Read local temperature from DS18B20
        let local_temp = gpio::read_temperature();

        // Process both remote and local data
        let remote_commands = controller::process_remote_readings(&readings, &config.controller_id);
        let local_commands = controller::process_local_readings(local_temp, &config.controller_id);
        let commands: Vec<controller::Command> = remote_commands.into_iter().chain(local_commands).collect();

        // Execute commands
        for cmd in commands {
            match cmd {
                controller::Command::TurnOnCooling(id) => {
                    info!("Turning on cooling for {}", id);
                    gpio::shift_out(0x01); // Set Q0 high (relay ON)
                }
                controller::Command::TurnOffCooling(id) => {
                    info!("Turning off cooling for {}", id);
                    gpio::shift_out(0x00); // Set Q0 low (relay OFF)
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
