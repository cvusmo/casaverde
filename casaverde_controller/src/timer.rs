// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/timer.rs

use crate::client::send_commands;
use crate::controller::Command;
use crate::serial::send_command;
use log::{error, info};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::{sync::Mutex, time::interval};
use reqwest::Client;

pub async fn run_light_timer(
    relay_id: String,
    on_hours: u64,
    off_hours: u64,
    port: Arc<Mutex<Box<dyn serialport::SerialPort>>>,
    client: Client,
    server: String,
) {
    info!("Starting light timer for relay {} with {}h ON / {}h OFF cycle", relay_id, on_hours, off_hours);

    // Set initial state based on current time
    let now = SystemTime::now();
    let hour = now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() / 3600 % 24;
    let initial_cmd = if hour < 18 {
        Command::TurnOnSolar(relay_id.clone())
    } else {
        Command::TurnOffSolar(relay_id.clone())
    };
    if let Err(e) = send_commands(&client, &server, &[initial_cmd.clone()]).await {
        error!("Failed to send initial light command to server: {}", e);
    }
    {
        let mut port_guard = port.lock().await;
        if let Err(e) = send_command(&mut **port_guard, &initial_cmd) {
            error!("Failed to send initial light command to serial: {}", e);
        } else {
            info!("Set initial light state to {} at hour {}", if hour < 18 { "ON" } else { "OFF" }, hour);
        }
    }

    // Testing cycle: 15s ON, 15s OFF, repeat every 30s
    let mut interval = interval(Duration::from_secs(15));
    let mut is_on = hour < 18; 

    loop {
        interval.tick().await;
        is_on = !is_on;
        let cmd = if is_on {
            Command::TurnOnSolar(relay_id.clone())
        } else {
            Command::TurnOffSolar(relay_id.clone())
        };
        if let Err(e) = send_commands(&client, &server, &[cmd.clone()]).await {
            error!("Failed to send light {} to server: {}", if is_on { "ON" } else { "OFF" }, e);
        }
        {
            let mut port_guard = port.lock().await;
            if let Err(e) = send_command(&mut **port_guard, &cmd) {
                error!("Failed to send light {} to serial: {}", if is_on { "ON" } else { "OFF" }, e);
            } else {
                info!("Toggled light to {} at {}", if is_on { "ON" } else { "OFF" }, SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
            }
        }
    }
}
