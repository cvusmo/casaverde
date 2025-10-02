// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs - Entry point for casaverde_controller

use log::{info, warn, error};
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use env_logger::{Builder, Target};

use casaverde_controller::config;
use casaverde_controller::client;
use casaverde_controller::controller::{Command, process_remote_readings};
use casaverde_controller::gpio;
use casaverde_controller::serial::{init_serial, send_command};

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
    //let mut port = init_serial(&config)?;
    let port = Arc::new(Mutex::new(init_serial(&config)?));

    // TODO: config loading for controller which is going to be checked. It will be saved on the
    // server (the config) and the app can modify the config for the controller and then save it
    // back on the server, in which case the controller will check for an update periodically
    // (every 24 hours or when starting)
    // FIXME: Need to have variables set in a config, and main retrieves the values and sets them.
    let timer_port = port.clone();
    let timer_client = client.clone();
    let timer_server = config.server.clone();
    let light_id = config.light_relay_id.clone();
    let on_hours = config.light_on_hours;
    let off_hours = config.light_off_hours;
    
    loop {
        // Fetch from casaverde_server
        let readings = client::fetch_readings(&client, &config.server).await?;
        info!("Fetched readings from server: {readings:?}");
        
        // Read local temperature
        let local_temp = gpio::read_temperature();
        info!("Local temperature: {local_temp:?}");

        // Process data
        let remote_commands = process_remote_readings(&readings, &config.controller_id);
        //let local_commands = process_local_readings(local_temp, &config.controller_id);
        //let commands: Vec<Command> = remote_commands.into_iter().chain(local_commands).collect();
        let commands: Vec<Command> = remote_commands.into_iter().collect();
        info!("Generated commands: {commands:?}");

        client::send_commands(&client, &config.server, &commands).await?;
        for cmd in &commands {
            info!("Executing command: {cmd:?}");
            send_command(&mut *port, cmd)?;
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
