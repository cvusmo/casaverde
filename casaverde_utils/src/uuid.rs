// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// src/uuid.rs

use crate::time::current_timestamp;
use std::sync::atomic::{AtomicU64, Ordering};

/// Generates a simple unique ID based on timestamp and a counter.
pub fn generate_id(prefix: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let timestamp = current_timestamp();
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}", prefix, timestamp, count)
}
