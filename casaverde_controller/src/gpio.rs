// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/gpio.rs

use std::fs::{read_dir, File};
use std::io::Read;
use std::path::Path;

pub fn read_temperature() -> Option<f32> {
    let thermal_dir = Path::new("/sys/class/hwmon/");
    if let Ok(entries) = read_dir(thermal_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name_path = path.join("name");
                let mut name = String::new();
                if let Ok(mut file) = File::open(&name_path) {
                    if file.read_to_string(&mut name).is_ok() && name.trim() == "coretemp" {
                        let mut i = 1;
                        loop {
                            let label_path = path.join(format!("temp{}_label", i));
                            if !label_path.exists() {
                                break;
                            }
                            let mut label = String::new();
                            if let Ok(mut file) = File::open(&label_path) {
                                if file.read_to_string(&mut label).is_ok()
                                    && label.trim() == "Package id 0"
                                {
                                    let temp_path = path.join(format!("temp{}_input", i));
                                    let mut temp_str = String::new();
                                    if let Ok(mut file) = File::open(&temp_path) {
                                        if file.read_to_string(&mut temp_str).is_ok() {
                                            if let Ok(temp_milli) = temp_str.trim().parse::<f32>() {
                                                return Some(temp_milli / 1000.0);
                                            }
                                        }
                                    }
                                }
                            }
                            i += 1;
                        }
                    }
                }
            }
        }
    }
    None
}
