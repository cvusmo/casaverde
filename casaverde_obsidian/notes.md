lets review the logs and then work on the short term:
Short-term (Minimal Changes):Use localhost for all: Set server = "https://127.0.0.1:3003" in all configs. This avoids external network latency (current is "10.0.0.6", which might be LAN).
Reduce intervals: Controller's 5s tick and app's 1s sleep are fine, but align them (e.g., 5s everywhere) to minimize fetches. Add cache expiry in server (e.g., ignore data >30s old) to prevent stale displays.
Fix serial reading: Modify read_sensor_data to read lines iteratively (use a loop with read_until('\n') or split the buffer by '\n' and parse each). This handles multiple Arduino prints without failing.
Disable unnecessary sends: In app's update_devices, remove the POST if >5s—it's redundant on one device.
Profile: Use logs/timings (add Instant::elapsed()) in loops to identify bottlenecks (e.g., serial reads or HTTP).

build.log:
[2025-10-05 01:42:13] Building casaverde_server for x86_64-unknown-linux-gnu in debug mode...
   Compiling proc-macro2 v1.0.101
   ...
   Compiling casaverde_utils v0.1.2 (/home/echo/projects/remote/casaverde/casaverde_utils)
warning: unused import: `std::fs::File`
 --> casaverde_utils/src/lib.rs:4:5
  |
4 | use std::fs::File;
  |     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `std::path::Path`
 --> casaverde_utils/src/lib.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `tokio::time::Instant`
 --> casaverde_utils/src/lib.rs:7:5
  |
7 | use tokio::time::Instant;
  |     ^^^^^^^^^^^^^^^^^^^^

warning: `casaverde_utils` (lib) generated 3 warnings (run `cargo fix --lib -p casaverde_utils` to apply 3 suggestions)
   Compiling tower-http v0.6.6
   Compiling h2 v0.4.12
   Compiling rustls-webpki v0.103.7
   Compiling hyper v1.7.0
   Compiling hyper-util v0.1.17
   Compiling tokio-rustls v0.26.4
   Compiling hyper-rustls v0.27.7
   Compiling axum v0.7.9
   Compiling axum-server v0.6.0
   Compiling reqwest v0.12.23
   Compiling casaverde_server v0.1.2 (/home/echo/projects/remote/casaverde/casaverde_server)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.96s
[2025-10-05 01:42:23] Built casaverde_server successfully
[2025-10-05 01:42:23] Building casaverde_app for x86_64-unknown-linux-gnu in debug mode...
   Compiling libc v0.2.176
   ...
   Compiling casaverde_utils v0.1.2 (/home/echo/projects/remote/casaverde/casaverde_utils)
warning: unused import: `std::fs::File`
 --> casaverde_utils/src/lib.rs:4:5
  |
4 | use std::fs::File;
  |     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `std::path::Path`
 --> casaverde_utils/src/lib.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `tokio::time::Instant`
 --> casaverde_utils/src/lib.rs:7:5
  |
7 | use tokio::time::Instant;
  |     ^^^^^^^^^^^^^^^^^^^^

warning: `casaverde_utils` (lib) generated 3 warnings (run `cargo fix --lib -p casaverde_utils` to apply 3 suggestions)
   Compiling tower-http v0.6.6
   ...
   Compiling casaverde_app v0.1.2 (/home/echo/projects/remote/casaverde/casaverde_app)
warning: unused import: `uuid::Uuid`
 --> casaverde_app/src/devices.rs:8:5
  |
8 | use uuid::Uuid;
  |     ^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `casaverde_app` (lib) generated 1 warning (run `cargo fix --lib -p casaverde_app` to apply 1 suggestion)
warning: unused import: `run_app`
 --> casaverde_app/src/main.rs:7:26
  |
7 | use casaverde_app::app::{run_app, CasaverdeApp};
  |                          ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `casaverde_app` (bin "casaverde_app") generated 1 warning (run `cargo fix --bin "casaverde_app"` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.82s
[2025-10-05 01:42:29] Built casaverde_app successfully
[2025-10-05 01:42:29] Building casaverde_controller for x86_64-unknown-linux-gnu in debug mode...
   Compiling cc v1.2.40
   ...
   Compiling casaverde_utils v0.1.2 (/home/echo/projects/remote/casaverde/casaverde_utils)
warning: unused import: `std::fs::File`
 --> casaverde_utils/src/lib.rs:4:5
  |
4 | use std::fs::File;
  |     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `std::path::Path`
 --> casaverde_utils/src/lib.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `tokio::time::Instant`
 --> casaverde_utils/src/lib.rs:7:5
  |
7 | use tokio::time::Instant;
  |     ^^^^^^^^^^^^^^^^^^^^

warning: `casaverde_utils` (lib) generated 3 warnings (run `cargo fix --lib -p casaverde_utils` to apply 3 suggestions)
   Compiling tower-http v0.6.6
   ...
   Compiling casaverde_controller v0.1.2 (/home/echo/projects/remote/casaverde/casaverde_controller)
warning: unused import: `log::info`
 --> casaverde_controller/src/controller.rs:5:5
  |
5 | use log::info;
  |     ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `tokio::time::interval`
 --> casaverde_controller/src/controller.rs:7:5
  |
7 | use tokio::time::interval;
  |     ^^^^^^^^^^^^^^^^^^^^^

warning: `casaverde_controller` (lib) generated 2 warnings (run `cargo fix --lib -p casaverde_controller` to apply 2 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.63s
[2025-10-05 01:42:33] Built casaverde_controller successfully
[2025-10-05 01:42:33] debug build complete

casaverde_server.log:
2025-10-05 01:44:45 [INFO] - casaverde_utils: Logger initialized for casaverde_server at level Info with log file /home/echo/projects/remote/casaverde/build_output/linux/casaverde_server/logs/casaverde_server.log
2025-10-05 01:44:45 [INFO] - casaverde_server: Starting casaverde_server
2025-10-05 01:44:45 [INFO] - casaverde_server: Server running on https://127.0.0.1:3003

casaverde_controller.log:
2025-10-05 01:46:18 [INFO] - casaverde_utils: Logger initialized for casaverde_controller at level Info with log file /home/echo/projects/remote/casaverde/build_output/linux/casaverde_controller/logs/casaverde_controller.log
2025-10-05 01:46:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:46:18 [INFO] - casaverde_controller: Starting casaverde_controller on unknown
2025-10-05 01:46:18 [INFO] - casaverde_controller::config: Configuration loaded successfully
2025-10-05 01:46:18 [INFO] - casaverde_controller::client: Certificate loaded successfully
2025-10-05 01:46:18 [INFO] - casaverde_controller::serial: Serial port /dev/ttyACM0 initialized at 9600 baud
2025-10-05 01:46:18 [INFO] - casaverde_controller: Failed to fetch config from server; using local config.toml: Config { server: "127.0.0.1:3003", controller_id: "blackbeard-pi", serial_port: Some("/dev/ttyACM0"), light_relay_id: "relay-1", light_on_hours: 16, light_off_hours: 8 }
2025-10-05 01:46:18 [INFO] - casaverde_controller::timer: Starting light timer with 16h ON / 8h OFF cycle
2025-10-05 01:46:18 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8172, tv_nsec: 524917776 }
2025-10-05 01:46:18 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:46:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:46:18 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:46:19 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:19 [INFO] - casaverde_controller::serial: Sent command on device solar-1

casaverde_app.log:
2025-10-05 01:46:37 [INFO] - casaverde_utils: Logger initialized for casaverde_app at level Info with log file /home/echo/projects/remote/casaverde/build_output/linux/casaverde_app/logs/casaverde_app.log
2025-10-05 01:46:37 [INFO] - casaverde_app: Starting Casaverde application
2025-10-05 01:46:37 [INFO] - casaverde_app::devices: DeviceData initialized with 7 devices
2025-10-05 01:46:37 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:38 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:39 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:41 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:42 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:43 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items

That is the logs as they started. heres the logs as ive let the server, controller, and app run for some time:

casaverde_app.log:
2025-10-05 01:46:37 [INFO] - casaverde_utils: Logger initialized for casaverde_app at level Info with log file /home/echo/projects/remote/casaverde/build_output/linux/casaverde_app/logs/casaverde_app.log
2025-10-05 01:46:37 [INFO] - casaverde_app: Starting Casaverde application
2025-10-05 01:46:37 [INFO] - casaverde_app::devices: DeviceData initialized with 7 devices
2025-10-05 01:46:37 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:37 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:38 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:38 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:39 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:39 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:41 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:41 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:42 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:42 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:43 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:43 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:44 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:44 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:44 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:44 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:44 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:44 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:44 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:44 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:44 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:45 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:45 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:45 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:45 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:45 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:45 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:45 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:45 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:45 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:47 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:47 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:47 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:47 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:47 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:47 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:47 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:47 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:47 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:48 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:48 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:48 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:48 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:48 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:48 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:48 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:48 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:48 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:49 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:49 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:49 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:49 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:49 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:49 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:49 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:49 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:49 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:50 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:50 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:50 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:50 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:50 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:50 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:50 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:50 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:50 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:51 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:51 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:51 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:51 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:51 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:51 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:51 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:51 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:51 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:53 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:53 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:53 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:53 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:53 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:53 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:53 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:53 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:53 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:54 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:54 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:54 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:54 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:54 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:54 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:54 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:54 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:54 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:55 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:55 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:55 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:55 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:55 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:55 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:55 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:55 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:55 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:56 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:56 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:56 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:56 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:56 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:56 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:56 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:56 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:56 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:57 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:57 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:57 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:57 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:57 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:57 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:57 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:57 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:57 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:46:59 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:46:59 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:46:59 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:46:59 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:46:59 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:46:59 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:46:59 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:46:59 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:46:59 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:00 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:00 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:00 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:00 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:00 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:00 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:00 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:00 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:00 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:01 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:01 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:01 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:01 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:01 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:01 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:01 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:01 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:01 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:02 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:02 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:02 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:02 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:02 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:02 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:02 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:02 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:02 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:03 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:03 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:03 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:03 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:03 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:03 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:03 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:03 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:03 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:05 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:05 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:05 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:05 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:05 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:05 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:05 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:05 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:05 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:06 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:06 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:06 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:06 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:06 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:06 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:06 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:06 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:06 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:07 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:07 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:07 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:07 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:07 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:07 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:07 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:07 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:07 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:08 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:08 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:08 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:08 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:08 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:08 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:08 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:08 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:08 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:09 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:09 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:09 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:09 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:09 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:09 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:09 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:09 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:09 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:11 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:11 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:11 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:11 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:11 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:11 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:11 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:11 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:11 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:12 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:12 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:12 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:12 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:12 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:12 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:12 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:12 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:12 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:13 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:13 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:13 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:13 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:13 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:13 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:13 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:13 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:13 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:14 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:14 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:14 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:14 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:14 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:14 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:14 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:14 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:14 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:15 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:15 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:15 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:15 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:15 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:15 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:15 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:15 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:15 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:17 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:17 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:17 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:17 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:17 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:17 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:17 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:17 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:17 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:18 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:18 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:18 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:18 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:18 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:18 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:18 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:18 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:18 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:19 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:19 [INFO] - casaverde_app::tui: Switched to Monitoring screen
2025-10-05 01:47:20 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:20 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:20 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:20 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:20 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:20 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:20 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:20 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:20 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:47:21 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:21 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:21 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:21 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:21 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:21 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:21 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:21 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:21 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:47:22 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:22 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:22 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:22 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:22 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:22 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:22 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:22 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:22 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:47:24 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:47:24 [INFO] - casaverde_app::tui: Switched to Monitoring screen
2025-10-05 01:47:25 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:25 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:47:26 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:26 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:47:27 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:27 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:47:28 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:28 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:47:30 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:30 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:47:31 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:31 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:47:31 [INFO] - casaverde_app::tui: Switched to Monitoring screen
2025-10-05 01:47:32 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:32 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:32 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:32 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:32 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:32 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:32 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:32 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:32 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:33 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:33 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:33 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:33 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:33 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:33 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:33 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:33 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:33 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:34 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:34 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:34 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:34 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:34 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:34 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:34 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:34 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:34 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:35 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:35 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:35 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:35 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:35 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:35 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:35 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:35 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:35 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:36 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:36 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:36 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:36 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:36 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:36 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:36 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:36 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:36 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:37 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:37 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:37 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:37 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:37 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:37 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:37 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:37 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:37 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:38 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:38 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:38 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:38 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:38 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:38 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:38 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:38 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:38 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:39 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:39 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:39 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:39 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:39 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:39 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:39 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:39 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:39 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:40 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:40 [INFO] - casaverde_app::devices: Toggled Probe Temperature to false
2025-10-05 01:47:40 [INFO] - casaverde_app::tui: Toggled selected sensor with Enter
2025-10-05 01:47:41 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:41 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:41 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:41 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:41 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:41 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:41 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:41 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:41 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:42 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:42 [INFO] - casaverde_app::devices: Toggled Probe Temperature to true
2025-10-05 01:47:42 [INFO] - casaverde_app::tui: Toggled selected sensor with Enter
2025-10-05 01:47:43 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:43 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:43 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:43 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:43 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:43 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:43 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:43 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:43 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:45 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:45 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:45 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:45 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:45 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:45 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:45 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:45 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:45 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:46 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:46 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:46 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:46 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:46 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:46 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:46 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:46 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:46 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:47 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:47 [INFO] - casaverde_app::tui: Toggled selected sensor with Enter
2025-10-05 01:47:48 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:48 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:48 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:48 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:48 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:48 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:48 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:48 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:48 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:49 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:49 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:49 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:49 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:49 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:49 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:49 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:49 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:49 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:50 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:50 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:50 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:50 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:50 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:50 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:50 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:50 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:50 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:51 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:51 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:51 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:51 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:51 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:51 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:51 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:51 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:51 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:53 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:47:53 [INFO] - casaverde_app::tui: Switched to Monitoring screen
2025-10-05 01:47:54 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:54 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:54 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:54 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:54 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:54 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:54 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:54 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:54 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:47:55 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:55 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:55 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:55 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:55 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:55 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:55 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:55 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:55 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:47:56 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:56 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:56 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:56 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:56 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:56 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:56 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:56 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:56 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:47:57 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:57 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:57 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:57 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:57 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:57 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:57 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:57 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:57 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:47:58 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:47:58 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:47:58 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:47:58 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:47:58 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:47:58 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:47:58 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:47:58 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:47:58 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:00 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:00 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:00 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:00 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:00 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:00 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:00 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:00 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:00 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:01 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:01 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:01 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:01 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:01 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:01 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:01 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:01 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:01 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:02 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:02 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:02 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:02 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:02 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:02 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:02 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:02 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:02 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:03 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:03 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:03 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:03 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:03 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:03 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:03 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:03 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:03 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:04 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:04 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:04 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:04 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:04 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:04 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:04 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:04 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:04 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:06 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:06 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:06 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:06 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:06 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:06 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:06 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:06 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:06 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:07 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:07 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:07 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:07 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:07 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:07 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:07 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:07 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:07 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:08 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:08 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:08 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:08 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:08 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:08 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:08 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:08 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:08 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:09 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:09 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:09 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:09 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:09 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:09 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:09 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:09 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:09 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:10 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:10 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:10 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:10 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:10 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:10 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:10 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:10 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:10 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:12 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:12 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:12 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:12 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:12 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:12 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:12 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:12 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:12 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:13 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:13 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:13 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:13 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:13 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:13 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:13 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:13 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:13 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:14 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:14 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:14 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:14 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:14 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:14 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:14 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:14 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:14 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:15 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:15 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:15 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:15 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:15 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:15 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:15 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:15 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:15 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:16 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:16 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:16 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:16 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:16 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:16 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:16 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:16 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:16 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:18 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:18 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:18 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:18 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:18 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:18 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:18 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:18 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:18 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:19 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:19 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:19 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:19 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:19 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:19 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:19 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:19 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:19 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:20 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:20 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:20 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:20 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:20 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:20 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:20 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:20 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:20 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:21 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:21 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:21 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:21 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:21 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:21 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:21 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:21 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:21 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:22 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:22 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:22 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:22 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:22 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:22 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:22 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:22 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:22 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:24 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Monitoring device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Monitoring device 1: id=solar-1, value=None
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Monitoring device 2: id=moisture-1, value=None
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Monitoring device 3: id=humidity-1, value=None
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Monitoring device 4: id=water-1, value=None
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Monitoring device 5: id=relay-1, value=None
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Monitoring device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Rendered Monitoring screen with 7 items
2025-10-05 01:48:24 [INFO] - casaverde_app::tui: Switched to Monitoring screen
2025-10-05 01:48:25 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:25 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:48:26 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:26 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:48:27 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:27 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:48:28 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:28 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:48:30 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:30 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:48:31 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:31 [INFO] - casaverde_app::tui: Rendered Config screen with 9 items
2025-10-05 01:48:31 [INFO] - casaverde_app::tui: Switched to Monitoring screen
2025-10-05 01:48:32 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:32 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:32 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:32 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:32 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:32 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:32 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:32 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:32 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:33 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:33 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:33 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:33 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:33 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:33 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:33 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:33 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:33 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:34 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:34 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:34 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:34 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:34 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:34 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:34 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:34 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:34 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:35 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:35 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:35 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:35 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:35 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:35 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:35 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:35 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:35 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:37 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:37 [INFO] - casaverde_app::tui: Toggled selected sensor with Enter
2025-10-05 01:48:38 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:38 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:38 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:38 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:38 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:38 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:38 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:38 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:38 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:39 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:39 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:39 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:39 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:39 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:39 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:39 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:39 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:39 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:40 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:40 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:40 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:40 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:40 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:40 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:40 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:40 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:40 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:41 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:41 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:41 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:41 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:41 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:41 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:41 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:41 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:41 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:42 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:42 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:42 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:42 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:42 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:42 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:42 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:42 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:42 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:44 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:44 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:44 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:44 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:44 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:44 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:44 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:44 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:44 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:45 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:45 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:45 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:45 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:45 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:45 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:45 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:45 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:45 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:46 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:46 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:46 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:46 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:46 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:46 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:46 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:46 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:46 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:47 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:47 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:47 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:47 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:47 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:47 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:47 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:47 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:47 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:48 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:48 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:48 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:48 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:48 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:48 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:48 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:48 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:48 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:50 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:50 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:50 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:50 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:50 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:50 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:50 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:50 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:50 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:51 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:51 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:51 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:51 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:51 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:51 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:51 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:51 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:51 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:52 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:52 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:52 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:52 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:52 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:52 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:52 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:52 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:52 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:53 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:53 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:53 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:53 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:53 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:53 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:53 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:53 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:53 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:54 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:54 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:54 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:54 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:54 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:54 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:54 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:54 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:54 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:56 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:56 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:56 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:56 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:56 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:56 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:56 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:56 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:56 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:57 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:57 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:57 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:57 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:57 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:57 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:57 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:57 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:57 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:58 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:58 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:58 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:58 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:58 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:58 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:58 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:58 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:58 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:48:59 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:48:59 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:48:59 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:48:59 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:48:59 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:48:59 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:48:59 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:48:59 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:48:59 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:00 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:00 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:00 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:00 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:00 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:00 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:00 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:00 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:00 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:02 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:02 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:02 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:02 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:02 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:02 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:02 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:02 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:02 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:03 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:03 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:03 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:03 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:03 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:03 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:03 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:03 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:03 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:04 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:04 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:04 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:04 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:04 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:04 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:04 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:04 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:04 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:05 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:05 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:05 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:05 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:05 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:05 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:05 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:05 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:05 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:06 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:06 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:06 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:06 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:06 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:06 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:06 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:06 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:06 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:08 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:08 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:08 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:08 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:08 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:08 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:08 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:08 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:08 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:09 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:09 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:09 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:09 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:09 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:09 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:09 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:09 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:09 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:10 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:10 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:10 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:10 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:10 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:10 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:10 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:10 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:10 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:11 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:11 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:11 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:11 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:11 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:11 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:11 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:11 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:11 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:12 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:12 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:12 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:12 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:12 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:12 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:12 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:12 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:12 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:14 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:14 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:14 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:14 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:14 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:14 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:14 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:14 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:14 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:15 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:15 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:15 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:15 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:15 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:15 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:15 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:15 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:15 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:16 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:16 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:16 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:16 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:16 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:16 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:16 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:16 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:16 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:17 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:17 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:17 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:17 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:17 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:17 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:17 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:17 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:17 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:18 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:18 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:18 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:18 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:18 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:18 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:18 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:18 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:18 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:20 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:20 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:20 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:20 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:20 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:20 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:20 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:20 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:20 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:21 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:21 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:21 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:21 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:21 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:21 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:21 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:21 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:21 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:22 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:22 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:22 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:22 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:22 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:22 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:22 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:22 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:22 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:23 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:23 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:23 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:23 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:23 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:23 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:23 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:23 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:23 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:24 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:24 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:24 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:24 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:24 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:24 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:24 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:24 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:24 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:26 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:26 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:26 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:26 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:26 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:26 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:26 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:26 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:26 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:27 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:27 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:27 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:27 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:27 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:27 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:27 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:27 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:27 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:28 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:28 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:28 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:28 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:28 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:28 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:28 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:28 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:28 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:29 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:29 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:29 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:29 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:29 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:29 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:29 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:29 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:29 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:30 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:30 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:30 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:30 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:30 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:30 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:30 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:30 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:30 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:32 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:32 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:32 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:32 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:32 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:32 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:32 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:32 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:32 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:33 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:33 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:33 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:33 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:33 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:33 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:33 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:33 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:33 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:34 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:34 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:34 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:34 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:34 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:34 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:34 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:34 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:34 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:35 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:35 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:35 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:35 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:35 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:35 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:35 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:35 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:35 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:36 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:36 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:36 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:36 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:36 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:36 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:36 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:36 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:36 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:38 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:38 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:38 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:38 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:38 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:38 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:38 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:38 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:38 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:39 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:39 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:39 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:39 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:39 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:39 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:39 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:39 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:39 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:40 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:40 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:40 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:40 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:40 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:40 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:40 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:40 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:40 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:41 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:41 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:41 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:41 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:41 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:41 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:41 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:41 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:41 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:42 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:42 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:42 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:42 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:42 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:42 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:42 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:42 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:42 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:44 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:44 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:44 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:44 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:44 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:44 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:44 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:44 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:44 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:45 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:45 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:45 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:45 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:45 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:45 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:45 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:45 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:45 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:46 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:46 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:46 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:46 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:46 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:46 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:46 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:46 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:46 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:47 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:47 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:47 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:47 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:47 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:47 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:47 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:47 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:47 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:48 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:48 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:48 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:48 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:48 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:48 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:48 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:48 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:48 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:50 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:50 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:50 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:50 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:50 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:50 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:50 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:50 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:50 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:51 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:51 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:51 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:51 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:51 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:51 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:51 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:51 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:51 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:52 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:52 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:52 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:52 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:52 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:52 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:52 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:52 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:52 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:53 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:53 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:53 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:53 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:53 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:53 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:53 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:53 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:53 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:54 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:54 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:54 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:54 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:54 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:54 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:54 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:54 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:54 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:56 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:56 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:56 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:56 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:56 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:56 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:56 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:56 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:56 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:57 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:57 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:57 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:57 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:57 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:57 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:57 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:57 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:57 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:58 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:58 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:58 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:58 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:58 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:58 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:58 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:58 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:58 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:49:59 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:49:59 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:49:59 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:49:59 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:49:59 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:49:59 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:49:59 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:49:59 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:49:59 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:00 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:00 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:00 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:00 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:00 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:00 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:00 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:00 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:00 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:02 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:02 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:02 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:02 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:02 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:02 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:02 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:02 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:02 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:03 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:03 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:03 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:03 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:03 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:03 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:03 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:03 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:03 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:04 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:04 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:04 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:04 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:04 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:04 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:04 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:04 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:04 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:05 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:05 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:05 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:05 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:05 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:05 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:05 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:05 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:05 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:06 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:06 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:06 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:06 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:06 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:06 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:06 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:06 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:06 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:08 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:08 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:08 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:08 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:08 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:08 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:08 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:08 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:08 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:09 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:09 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:09 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:09 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:09 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:09 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:09 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:09 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:09 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:10 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:10 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:10 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:10 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:10 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:10 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:10 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:10 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:10 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:11 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:11 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:11 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:11 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:11 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:11 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:11 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:11 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:11 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:12 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:12 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:12 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:12 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:12 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:12 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:12 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:12 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:12 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:14 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:14 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:14 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:14 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:14 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:14 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:14 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:14 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:14 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:15 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:15 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:15 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:15 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:15 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:15 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:15 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:15 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:15 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:16 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:16 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:16 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:16 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:16 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:16 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:16 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:16 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:16 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:17 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:17 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:17 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:17 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:17 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:17 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:17 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:17 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:17 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:18 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:18 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:18 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:18 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:18 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:18 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:18 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:18 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:18 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:20 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:20 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:20 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:20 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:20 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:20 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:20 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:20 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:20 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:21 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:21 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:21 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:21 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:21 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:21 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:21 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:21 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:21 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:22 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:22 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:22 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:22 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:22 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:22 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:22 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:22 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:22 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:23 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:23 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:23 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:23 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:23 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:23 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:23 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:23 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:23 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:25 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:25 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:25 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:25 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:25 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:25 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:25 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:25 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:25 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:26 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:26 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:26 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:26 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:26 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:26 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:26 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:26 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:26 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:27 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:27 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:27 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:27 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:27 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:27 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:27 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:27 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:27 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:28 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:28 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:28 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:28 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:28 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:28 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:28 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:28 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:28 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:29 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:29 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:29 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:29 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:29 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:29 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:29 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:29 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:29 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:31 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:31 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:31 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:31 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:31 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:31 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:31 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:31 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:31 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:32 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:32 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:32 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:32 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:32 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:32 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:32 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:32 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:32 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:33 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:33 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:33 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:33 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:33 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:33 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:33 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:33 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:33 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:34 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:34 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:34 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:34 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:34 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:34 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:34 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:34 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:34 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:35 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:35 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:35 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:35 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:35 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:35 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:35 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:35 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:35 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:37 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:37 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:37 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:37 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:37 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:37 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:37 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:37 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:37 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:38 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:38 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:38 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:38 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:38 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:38 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:38 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:38 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:38 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:39 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:39 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:39 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:39 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:39 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:39 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:39 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:39 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:39 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:40 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:40 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:40 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:40 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:40 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:40 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:40 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:40 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:40 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:41 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:41 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:41 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:41 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:41 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:41 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:41 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:41 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:41 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:43 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:43 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:43 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:43 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:43 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:43 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:43 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:43 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:43 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:44 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:44 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:44 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:44 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:44 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:44 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:44 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:44 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:44 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:45 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:45 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:45 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:45 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:45 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:45 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:45 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:45 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:45 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:46 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:46 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:46 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:46 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:46 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:46 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:46 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:46 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:46 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:47 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:47 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:47 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:47 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:47 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:47 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:47 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:47 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:47 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:49 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:49 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:49 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:49 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:49 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:49 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:49 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:49 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:49 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:50 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:50 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:50 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:50 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:50 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:50 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:50 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:50 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:50 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:51 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:51 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:51 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:51 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:51 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:51 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:51 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:51 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:51 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:52 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:52 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:52 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:52 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:52 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:52 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:52 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:52 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:52 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:53 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:53 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:53 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:53 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:53 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:53 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:53 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:53 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:53 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:55 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:55 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:55 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:55 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:55 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:55 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:55 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:55 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:55 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:56 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:56 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:56 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:56 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:56 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:56 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:56 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:56 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:56 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:57 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:57 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:57 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:57 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:57 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:57 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:57 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:57 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:57 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:58 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:58 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:58 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:58 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:58 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:58 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:58 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:58 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:58 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:50:59 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:50:59 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:50:59 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:50:59 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:50:59 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:50:59 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:50:59 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:50:59 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:50:59 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:01 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:01 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:01 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:01 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:01 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:01 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:01 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:01 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:01 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:02 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:02 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:02 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:02 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:02 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:02 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:02 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:02 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:02 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:03 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:03 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:03 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:03 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:03 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:03 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:03 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:03 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:03 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:04 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:04 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:04 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:04 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:04 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:04 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:04 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:04 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:04 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:05 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:05 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:05 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:05 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:05 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:05 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:05 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:05 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:05 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:07 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:07 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:07 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:07 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:07 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:07 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:07 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:07 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:07 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:08 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:08 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:08 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:08 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:08 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:08 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:08 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:08 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:08 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:09 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:09 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:09 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:09 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:09 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:09 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:09 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:09 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:09 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:10 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:10 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:10 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:10 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:10 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:10 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:10 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:10 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:10 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:11 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:11 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:11 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:11 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:11 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:11 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:11 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:11 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:11 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:13 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:13 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:13 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:13 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:13 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:13 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:13 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:13 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:13 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:14 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:14 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:14 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:14 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:14 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:14 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:14 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:14 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:14 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:15 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:15 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:15 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:15 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:15 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:15 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:15 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:15 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:15 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:16 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:16 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:16 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:16 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:16 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:16 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:16 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:16 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:16 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:17 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:17 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:17 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:17 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:17 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:17 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:17 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:17 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:17 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:19 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:19 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:19 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:19 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:19 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:19 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:19 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:19 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:19 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:20 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:20 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:20 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:20 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:20 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:20 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:20 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:20 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:20 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:21 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:21 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:21 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:21 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:21 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:21 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:21 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:21 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:21 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:22 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:22 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:22 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:22 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:22 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:22 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:22 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:22 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:22 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:23 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:23 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:23 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:23 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:23 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:23 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:23 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:23 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:23 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:25 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:25 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:25 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:25 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:25 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:25 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:25 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:25 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:25 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:26 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:26 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:26 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:26 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:26 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:26 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:26 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:26 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:26 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:27 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:27 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:27 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:27 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:27 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:27 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:27 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:27 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:27 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:28 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:28 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:28 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:28 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:28 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:28 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:28 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:28 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:28 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:29 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:29 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:29 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:29 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:29 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:29 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:29 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:29 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:29 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:31 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:31 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:31 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:31 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:31 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:31 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:31 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:31 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:31 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:32 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:32 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:32 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:32 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:32 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:32 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:32 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:32 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:32 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:33 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:33 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:33 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:33 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:33 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:33 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:33 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:33 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:33 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:34 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:34 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:34 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:34 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:34 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:34 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:34 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:34 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:34 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:35 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:35 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:35 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:35 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:35 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:35 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:35 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:35 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:35 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:37 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:37 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:37 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:37 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:37 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:37 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:37 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:37 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:37 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:38 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:38 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:38 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:38 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:38 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:38 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:38 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:38 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:38 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:39 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:39 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:39 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:39 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:39 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:39 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:39 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:39 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:39 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:40 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:40 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:40 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:40 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:40 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:40 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:40 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:40 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:40 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:41 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:41 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:41 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:41 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:41 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:41 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:41 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:41 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:41 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:43 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:43 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:43 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:43 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:43 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:43 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:43 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:43 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:43 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:44 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:44 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:44 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:44 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:44 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:44 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:44 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:44 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:44 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:45 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:45 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:45 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:45 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:45 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:45 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:45 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:45 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:45 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:46 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:46 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:46 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:46 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:46 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:46 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:46 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:46 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:46 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:48 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:48 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:48 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:48 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:48 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:48 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:48 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:48 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:48 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:49 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:49 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:49 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:49 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:49 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:49 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:49 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:49 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:49 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:50 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:50 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:50 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:50 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:50 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:50 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:50 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:50 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:50 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:51 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:51 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:51 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:51 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:51 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:51 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:51 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:51 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:51 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:52 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:52 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:52 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:52 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:52 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:52 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:52 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:52 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:52 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:54 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:54 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:54 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:54 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:54 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:54 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:54 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:54 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:54 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:55 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:55 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:55 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:55 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:55 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:55 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:55 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:55 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:55 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:56 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:56 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:56 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:56 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:56 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:56 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:56 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:56 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:56 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:57 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:57 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:57 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:57 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:57 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:57 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:57 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:57 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:57 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:51:58 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:51:58 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:51:58 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:51:58 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:51:58 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:51:58 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:51:58 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:51:58 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:51:58 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:00 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:00 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:00 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:00 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:00 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:00 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:00 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:00 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:00 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:01 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:01 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:01 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:01 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:01 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:01 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:01 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:01 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:01 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:02 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:02 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:02 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:02 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:02 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:02 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:02 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:02 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:02 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:03 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:03 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:03 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:03 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:03 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:03 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:03 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:03 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:03 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:04 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:04 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:04 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:04 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:04 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:04 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:04 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:04 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:04 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:06 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:06 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:06 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:06 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:06 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:06 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:06 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:06 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:06 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:07 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:07 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:07 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:07 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:07 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:07 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:07 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:07 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:07 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:08 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:08 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:08 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:08 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:08 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:08 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:08 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:08 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:08 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:09 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:09 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:09 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:09 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:09 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:09 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:09 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:09 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:09 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:10 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:10 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:10 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:10 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:10 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:10 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:10 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:10 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:10 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:12 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:12 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:12 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:12 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:12 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:12 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:12 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:12 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:12 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:13 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:13 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:13 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:13 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:13 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:13 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:13 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:13 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:13 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:14 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:14 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:14 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:14 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:14 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:14 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:14 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:14 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:14 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:15 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:15 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:15 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:15 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:15 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:15 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:15 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:15 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:15 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:16 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:16 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:16 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:16 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:16 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:16 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:16 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:16 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:16 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:18 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:18 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:18 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:18 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:18 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:18 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:18 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:18 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:18 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:19 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:19 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:19 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:19 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:19 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:19 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:19 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:19 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:19 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:20 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:20 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:20 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:20 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:20 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:20 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:20 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:20 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:20 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:21 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:21 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:21 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:21 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:21 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:21 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:21 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:21 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:21 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:22 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:22 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:22 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:22 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:22 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:22 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:22 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:22 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:22 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:24 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:24 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:24 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:24 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:24 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:24 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:24 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:24 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:24 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:25 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:25 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:25 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:25 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:25 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:25 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:25 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:25 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:25 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:26 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:26 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:26 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:26 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:26 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:26 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:26 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:26 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:26 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:27 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:27 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:27 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:27 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:27 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:27 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:27 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:27 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:27 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:28 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:28 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:28 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:28 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:28 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:28 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:28 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:28 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:28 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:30 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:30 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:30 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:30 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:30 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:30 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:30 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:30 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:30 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:31 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:31 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:31 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:31 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:31 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:31 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:31 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:31 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:31 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:32 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:32 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:32 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:32 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:32 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:32 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:32 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:32 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:32 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:33 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:33 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:33 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:33 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:33 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:33 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:33 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:33 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:33 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:34 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:34 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:34 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:34 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:34 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:34 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:34 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:34 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:34 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:36 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:36 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:36 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:36 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:36 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:36 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:36 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:36 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:36 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:37 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:37 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:37 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:37 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:37 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:37 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:37 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:37 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:37 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:38 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:38 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:38 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:38 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:38 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:38 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:38 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:38 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:38 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:39 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:39 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:39 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:39 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:39 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:39 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:39 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:39 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:39 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:40 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:40 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:40 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:40 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:40 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:40 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:40 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:40 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:40 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:42 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:42 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:42 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:42 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:42 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:42 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:42 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:42 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:42 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:43 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:43 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:43 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:43 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:43 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:43 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:43 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:43 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:43 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:44 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:44 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:44 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:44 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:44 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:44 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:44 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:44 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:44 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:45 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:45 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:45 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:45 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:45 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:45 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:45 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:45 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:45 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:46 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:46 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:46 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:46 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:46 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:46 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:46 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:46 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:46 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:48 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:48 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:48 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:48 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:48 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:48 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:48 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:48 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:48 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:49 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:49 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:49 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:49 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:49 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:49 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:49 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:49 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:49 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:50 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:50 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:50 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:50 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:50 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:50 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:50 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:50 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:50 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:51 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:51 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:51 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:51 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:51 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:51 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:51 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:51 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:51 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:52 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:52 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:52 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:52 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:52 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:52 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:52 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:52 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:52 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items
2025-10-05 01:52:54 [ERROR] - casaverde_app::devices: Failed to fetch sensor data from 127.0.0.1:3003/temps: builder error
2025-10-05 01:52:54 [INFO] - casaverde_app::tui: Rendering device 0: id=blackbeard-cpu, value=None
2025-10-05 01:52:54 [INFO] - casaverde_app::tui: Rendering device 1: id=solar-1, value=None
2025-10-05 01:52:54 [INFO] - casaverde_app::tui: Rendering device 2: id=moisture-1, value=None
2025-10-05 01:52:54 [INFO] - casaverde_app::tui: Rendering device 3: id=humidity-1, value=None
2025-10-05 01:52:54 [INFO] - casaverde_app::tui: Rendering device 4: id=water-1, value=None
2025-10-05 01:52:54 [INFO] - casaverde_app::tui: Rendering device 5: id=relay-1, value=None
2025-10-05 01:52:54 [INFO] - casaverde_app::tui: Rendering device 6: id=blackbeard-probe, value=None
2025-10-05 01:52:54 [INFO] - casaverde_app::tui: Rendered Devices screen with 7 items

casaverde_controller.log:
2025-10-05 01:46:18 [INFO] - casaverde_utils: Logger initialized for casaverde_controller at level Info with log file /home/echo/projects/remote/casaverde/build_output/linux/casaverde_controller/logs/casaverde_controller.log
2025-10-05 01:46:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:46:18 [INFO] - casaverde_controller: Starting casaverde_controller on unknown
2025-10-05 01:46:18 [INFO] - casaverde_controller::config: Configuration loaded successfully
2025-10-05 01:46:18 [INFO] - casaverde_controller::client: Certificate loaded successfully
2025-10-05 01:46:18 [INFO] - casaverde_controller::serial: Serial port /dev/ttyACM0 initialized at 9600 baud
2025-10-05 01:46:18 [INFO] - casaverde_controller: Failed to fetch config from server; using local config.toml: Config { server: "127.0.0.1:3003", controller_id: "blackbeard-pi", serial_port: Some("/dev/ttyACM0"), light_relay_id: "relay-1", light_on_hours: 16, light_off_hours: 8 }
2025-10-05 01:46:18 [INFO] - casaverde_controller::timer: Starting light timer with 16h ON / 8h OFF cycle
2025-10-05 01:46:18 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8172, tv_nsec: 524917776 }
2025-10-05 01:46:18 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:46:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:46:18 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:46:19 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:19 [INFO] - casaverde_controller::serial: Sent command on device solar-1
2025-10-05 01:46:24 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:29 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:33 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8187, tv_nsec: 525307048 }
2025-10-05 01:46:33 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:46:33 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:46:33 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:46:34 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:34 [INFO] - casaverde_controller::serial: Sent command on device solar-1
2025-10-05 01:46:39 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:44 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:48 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8202, tv_nsec: 525298132 }
2025-10-05 01:46:48 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:46:48 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:46:48 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:46:49 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:50 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:46:54 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:46:59 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:03 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8217, tv_nsec: 525131215 }
2025-10-05 01:47:03 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:47:03 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:47:03 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:47:04 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:05 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:47:09 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:14 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:18 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8232, tv_nsec: 525161256 }
2025-10-05 01:47:18 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:47:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:47:18 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:47:19 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:20 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:47:24 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:29 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:33 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8247, tv_nsec: 525250242 }
2025-10-05 01:47:33 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:47:33 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:47:33 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:47:34 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:35 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:47:39 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:44 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:48 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8262, tv_nsec: 525126889 }
2025-10-05 01:47:48 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:47:48 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:47:48 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:47:49 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:50 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:47:54 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:47:59 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:03 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8277, tv_nsec: 525128284 }
2025-10-05 01:48:03 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:48:03 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:48:03 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:48:04 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:05 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:48:09 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:14 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:18 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8292, tv_nsec: 525116779 }
2025-10-05 01:48:18 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:48:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:48:18 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:48:19 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:20 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:48:24 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:29 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:33 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8307, tv_nsec: 525155979 }
2025-10-05 01:48:33 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:48:33 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:48:33 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:48:34 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:35 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:48:39 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:44 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:48 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8322, tv_nsec: 525227250 }
2025-10-05 01:48:48 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:48:48 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:48:48 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:48:49 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:50 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:48:54 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:48:59 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:03 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8337, tv_nsec: 525291173 }
2025-10-05 01:49:03 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:49:03 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:49:03 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:49:04 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:05 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:49:09 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:14 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:18 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8352, tv_nsec: 525160494 }
2025-10-05 01:49:18 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:49:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:49:18 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:49:19 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:49:20 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:24 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:29 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:33 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8367, tv_nsec: 525147177 }
2025-10-05 01:49:33 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:49:33 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:49:33 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:49:34 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:35 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:49:39 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:44 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:48 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8382, tv_nsec: 525217519 }
2025-10-05 01:49:48 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:49:48 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:49:48 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:49:49 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:50 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:49:54 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:49:59 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:03 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8397, tv_nsec: 525215382 }
2025-10-05 01:50:03 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:50:03 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:50:03 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:50:04 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:05 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:50:09 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:14 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:18 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8412, tv_nsec: 525106521 }
2025-10-05 01:50:18 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:50:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:50:18 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:50:19 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:20 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:50:24 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:29 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:33 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8427, tv_nsec: 525266370 }
2025-10-05 01:50:33 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:50:33 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:50:33 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:50:34 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:35 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:50:39 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:44 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:48 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8442, tv_nsec: 525246846 }
2025-10-05 01:50:48 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:50:48 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:50:48 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:50:49 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:50 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:50:54 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:50:59 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:03 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8457, tv_nsec: 525198654 }
2025-10-05 01:51:03 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:51:03 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:51:03 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:51:04 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:05 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:51:09 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:14 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:18 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8472, tv_nsec: 525165844 }
2025-10-05 01:51:18 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:51:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:51:18 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:51:19 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:20 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:51:24 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:29 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:33 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8487, tv_nsec: 525163386 }
2025-10-05 01:51:33 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:51:33 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:51:33 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:51:34 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:35 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:51:39 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:44 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:48 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8502, tv_nsec: 524533566 }
2025-10-05 01:51:48 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:51:48 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:51:48 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:51:49 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:50 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:51:54 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:51:59 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:03 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8517, tv_nsec: 525131290 }
2025-10-05 01:52:03 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:52:03 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:52:03 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:52:04 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:05 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:52:09 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:14 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:18 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8532, tv_nsec: 526992056 }
2025-10-05 01:52:18 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:52:18 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:52:18 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:52:19 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:20 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:52:24 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:29 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:33 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8547, tv_nsec: 525139262 }
2025-10-05 01:52:33 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:52:33 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:52:33 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:52:34 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:35 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:52:39 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:44 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:48 [INFO] - casaverde_controller::timer: Toggled light to ON at Instant { tv_sec: 8562, tv_nsec: 525096384 }
2025-10-05 01:52:48 [INFO] - casaverde_controller: Executing command via serial: TurnOnSolar
2025-10-05 01:52:48 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:52:48 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:52:49 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:50 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out
2025-10-05 01:52:54 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:52:59 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:53:03 [INFO] - casaverde_controller::timer: Toggled light to OFF at Instant { tv_sec: 8577, tv_nsec: 525058928 }
2025-10-05 01:53:03 [INFO] - casaverde_controller: Executing command via serial: TurnOffSolar
2025-10-05 01:53:03 [INFO] - casaverde_controller::config: HOSTNAME environment variable not set, using 'unknown'
2025-10-05 01:53:03 [ERROR] - casaverde_controller: send_commands to server error: builder error
2025-10-05 01:53:04 [ERROR] - casaverde_controller: fetch_reads error: builder error
2025-10-05 01:53:05 [ERROR] - casaverde_controller: Error sending command via serial: Operation timed out

casaverde_server.log:
2025-10-05 01:44:45 [INFO] - casaverde_utils: Logger initialized for casaverde_server at level Info with log file /home/echo/projects/remote/casaverde/build_output/linux/casaverde_server/logs/casaverde_server.log
2025-10-05 01:44:45 [INFO] - casaverde_server: Starting casaverde_server
2025-10-05 01:44:45 [INFO] - casaverde_server: Server running on https://127.0.0.1:3003