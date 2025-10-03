// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/timer.rs

use crate::controller::Command;
use log::info;
use tokio::sync::mpsc;
use tokio::time::interval;

pub async fn run_light_timer(
    _relay_id: String,
    on_hours: u64,
    off_hours: u64,
    tx: mpsc::Sender<Command>,
) {
    info!(
        "Starting light timer with {}h ON / {}h OFF cycle",
        on_hours, off_hours
    );
    let mut interval = interval(tokio::time::Duration::from_secs(15));
    let mut is_on = true;

    loop {
        interval.tick().await;
        let cmd = if is_on { Command::TurnOnSolar } else { Command::TurnOffSolar };
        if tx.send(cmd).await.is_err() {
            break;
        }
        info!(
            "Toggled light to {} at {:?}",
            if is_on { "ON" } else { "OFF" },
            tokio::time::Instant::now()
        );
        is_on = !is_on;
    }
}
