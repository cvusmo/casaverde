// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_time
// src/lib.rs

pub mod formatted;
pub mod timer;
pub mod timestamp;

pub use formatted::formatted;
pub use timer::delay_ms;
pub use timestamp::timestamp;

/// Initializes the time module.
pub fn init() {
    println!("[casaverde_time] Initialized module");
}
