// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// src/time.rs

use casaverde_time::formatted;
use casaverde_time::timestamp;
use casaverde_time::timer;

/// Returns the current timestamp in seconds since UNIX epoch.
pub fn current_timestamp() -> u64 {
    timestamp::timestamp()
}

/// Returns a formatted timestamp string in YYYY-MM-DD HH:MM:SS format.
pub fn formatted_timestamp() -> String {
    formatted::formatted()
}

/// Async delay function for use in async contexts.
pub async fn delay(milliseconds: u64) {
    timer::delay_ms(milliseconds).await;
}

/// Initializes the time module (for consistency with other modules).
pub fn init_time() {
    casaverde_time::init();
}
