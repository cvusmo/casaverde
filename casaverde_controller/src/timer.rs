// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/timer.rs

use crate::client::{send_commands, Client};
use crate::controller::Command;
use crate::serial::send_command;
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::{sync::Mutex, time::sleep};

pub async fn run_light_timer(
    relay_id: String,
    on_duration_hours: u64,
    off_duration_hours: u64,
    port: Arc<Mutex<Box<dyn serialport::SerialPort>>>,
    client: Client,
    server: String,
) {
    info!("Starting light timer with ID {}, {}h on / {}h off cycle", relay_id, on_duration_hours, off_duration_hours);
    
    loop {
        let cmd_on = Command::TurnOnLight(relay_id.clone());
        if let Err(e) = send_commands(&client, &server, &[cmd_on.clone()]).await {
            error!("Failed to send light ON to server: {}", e);
        }
        {
            let mut port_guard = port.lock().await;
            if let Err(e) = send_command(&mut **port_guard, &cmd_on) {
                error!("Failed to send light ON to serial: {}", e);
            }
        }
        sleep(Duration::from_secs(on_duration_hours * 3600)).await;
        
        let cmd_off = Command::TurnOffLight(relay_id.clone());
        if let Err(e) = send_commands(&client, &server, &[cmd_off.clone()]).await {
            error!("Failed to send light OFF to server: {}", e);
        }
        {
            let mut port_guard = port.lock().await;
            if let Err(e) = send_command(&mut **port_guard, &cmd_off) {
                error!("Failed to send light OFF to serial: {}", e);
            }
        }
        sleep(Duration::from_secs(off_duration_hours * 3600)).await;
    }
}
