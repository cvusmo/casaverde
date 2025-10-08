// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_time
// src/timer.rs

use std::time::Duration;
use tokio::time::sleep;

/// Async delay helper.
pub async fn delay_ms(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}
