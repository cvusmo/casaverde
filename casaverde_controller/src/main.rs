// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs - Entry point for casaverde_controller

use log::info;
use std::time::Duration;

use casaverde_controller::config;
use casaverde_controller::client;
use casaverde_controller::controller;
use casaverde_controller::serial;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting casaverde_controller on {}", config::get_hostname());

    let config = config::load_config()?;
    let client = client::build_secure_client()?;
    let mut serial_port = serial::init_serial("/dev/ttyACM0", 9600)?;

    loop {
        let readings = client::fetch_readings(&client, &config.server).await?;
        let commands = controller::process_readings(&readings, &config.controller_id);
        for cmd in commands {
            serial::send_command(&mut *serial_port, cmd)?;
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
