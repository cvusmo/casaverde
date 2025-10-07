// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/ui.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn create_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),      // Title
            Constraint::Percentage(81), // Sensors
            Constraint::Length(4),      // Status
            Constraint::Min(1),         // Spacer
        ])
        .split(area)
        .to_vec()
}
