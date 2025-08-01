// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/lib.rs

//! # casaverde_server library
//!
//! This library provides the core functionality for the casaverde_server, including data models,
//! HTTP handlers, and a caching system to manage system data from multiple clients. It is designed
//! to run on an internal network with hardcoded IP's derived from device UUIDs

pub mod cache;
pub mod handlers;
pub mod models;
