// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_lazy
// src/lib.rs

use std::sync::OnceLock;

/// Returns a static OnceLock instance for lazy initialization of a value.
/// The value must implement Send + Sync to be safe for static use.
pub fn once_lock<T: Send + Sync + 'static>() -> &'static OnceLock<T> {
    static ONCE: OnceLock<OnceLock<()>> = OnceLock::new();
    ONCE.get_or_init(|| OnceLock::new())
}
