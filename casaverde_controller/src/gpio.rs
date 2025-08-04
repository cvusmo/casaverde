// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/gpio.rs

// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/gpio.rs

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn initialize_gpio() {
    let mut gpio_export = File::create("/sys/class/gpio/export").unwrap();
    gpio_export.write_all(b"17\n").unwrap(); // DS pin
    gpio_export.write_all(b"27\n").unwrap(); // SHCP pin
    gpio_export.write_all(b"22\n").unwrap(); // STCP pin
    thread::sleep(Duration::from_millis(100)); // Wait for export

    let gpio_direction = |pin: &str| {
        let mut file = File::create(&format!("/sys/class/gpio/gpio{}/direction", pin)).unwrap();
        file.write_all(b"out").unwrap();
    };
    gpio_direction("17");
    gpio_direction("27");
    gpio_direction("22");
}

pub fn shift_out(data: u8) {
    let gpio_write = |pin: &str, value: u8| {
        let mut file = File::create(&format!("/sys/class/gpio/gpio{}/value", pin)).unwrap();
        file.write_all(&[value]).unwrap();
    };

    gpio_write("22", 0); // ST_CP low
    for i in (0..8).rev() {
        let bit = (data >> i) & 1;
        gpio_write("17", bit); // DS
        gpio_write("27", 1); // SH_CP high
        std::thread::sleep(Duration::from_micros(1));
        gpio_write("27", 0); // SH_CP low
    }
    gpio_write("22", 1); // ST_CP high
    std::thread::sleep(Duration::from_micros(1));
    gpio_write("22", 0); // ST_CP low
}

pub fn read_temperature() -> Option<f32> {
    let devices = std::fs::read_dir("/sys/bus/w1/devices").ok()?;
    for device in devices {
        let device = device.ok()?;
        let path = device.path();
        if path.ends_with("w1_slave") {
            let mut file = File::open(&path).ok()?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).ok()?;
            let lines: Vec<&str> = contents.split('\n').collect();
            if lines.len() > 1 {
                let temp_data = lines[1].split('=').nth(1)?;
                return Some(temp_data.parse::<f32>().unwrap_or(0.0) / 1000.0); // Convert to °C
            }
        }
    }
    None
}
