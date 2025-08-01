// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/ui.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn create_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Title
            Constraint::Percentage(80), // Sensors
            Constraint::Length(3),      // Status
            Constraint::Min(0),         // Spacer
        ])
        .split(area)
        .to_vec()
}
