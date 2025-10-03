// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/gpio.rs

use std::fs;
use std::path::Path;

pub fn read_temperature() -> Option<f32> {
    fs::read_dir(Path::new("/sys/class/hwmon/"))
        .ok()?
        .flatten()
        .find_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                let name = fs::read_to_string(path.join("name"))
                    .ok()?
                    .trim()
                    .to_string();
                if name == "coretemp" {
                    (1..).find_map(|i| {
                        let label_path = path.join(format!("temp{}_label", i));
                        if !label_path.exists() {
                            return None;
                        }
                        let label = fs::read_to_string(label_path).ok()?.trim().to_string();
                        if label == "Package id 0" {
                            let temp_str =
                                fs::read_to_string(path.join(format!("temp{}_input", i))).ok()?;
                            temp_str.trim().parse::<f32>().ok().map(|t| t / 1000.0)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
}
