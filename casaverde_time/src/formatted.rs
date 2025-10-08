// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_time
// src/formatted.rs

use super::timestamp;

/// Returns a formatted UTC timestamp string using std-only.
pub fn formatted() -> String {
    timestamp::unix_time_to_human_readable(timestamp::timestamp())
}
