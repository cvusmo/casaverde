Actual flow of sensor data in the program binaries:The sensor data flow is designed around a client-server architecture where the casaverde_controller acts as the primary data producer (reading from hardware), the casaverde_server acts as a caching intermediary, and the casaverde_app acts as the consumer (fetching and displaying data). Here's a step-by-step breakdown based on the code:Data Production (casaverde_controller):Every 5 seconds (via tokio::time::interval), the controller reads raw sensor data.Local CPU temperature is read directly from the system's GPIO/sysfs interface (gpio::read_temperature()), which parses /sys/class/hwmon/ files for "coretemp" Package id 0.
Probe temperature (and potentially other sensors) is read from the Arduino Uno R3 via serial communication (serial::read_sensor_data()). The Arduino autonomously prints data like TEMP_PROBE:26.5 every second in its loop. The controller reads the serial buffer (up to 128 bytes), parses it using match_response to extract values (e.g., mapping TEMP_PROBE: to blackbeard-probe with value 26.5), and stores it in a Vec<DeviceReading>.

Additional rules are applied (e.g., if probe temp > 15.0°C, generate Command::TurnOnRelay2; local CPU rules for cooling).
The local CPU temp is appended to the serial readings.
The combined data is packaged as a SensorReading (with client_id: "blackbeard-pi") and sent to the server via HTTP POST to /sensor_data (client::send_sensor_data()).

Data Caching and Serving (casaverde_server):On receiving the POST to /sensor_data, the server inserts the data into an in-memory cache (cache::insert_temp_cache()), keyed by client_id (e.g., "blackbeard-pi") with the devices vector and a timestamp.
The cache is a simple HashMap<String, (Vec<DeviceReading>, Instant)> using Arc<Mutex<...>> for thread-safety.
When queried (e.g., GET /temps or /sensor_data), the server returns the cached data as JSON: a vector of (client_id, Vec<DeviceReading>).

Data Consumption and Display (casaverde_app):In the app's update loop (update_devices()), it fetches data from the server via HTTP GET to /temps.
It parses the JSON into Vec<(String, Vec<DeviceReading>)>.
It filters for readings matching client_id == "blackbeard-pi" (hardcoded).
For each device in its config (from config.toml, e.g., "blackbeard-probe", "blackbeard-cpu"), it looks up the value by ID and stores it in device_values (a vec of Option<f32>) and temp_data (for CPU/probe specifically).
If the last update was >5 seconds ago, it sends back a subset of the data (only active/toggled sensors) via POST to /sensor_data (this seems redundant but echoes data back to the server for caching).
In the TUI rendering (render_tui()), it displays the values in the "Devices" or "Monitoring" screen, formatted (e.g., "Probe Temperature: 26.5°C") or "N/A" if None.

Notes on the flow:It's steady and periodic, but the serial reading in the controller assumes a single response per read. If the Arduino prints multiple lines (e.g., over 5 seconds), the buffer may contain concatenated data (e.g., "TEMP_PROBE:26.5\nTEMP_PROBE:26.4"), causing match_response to fail (it uses strip_prefix, which won't match multi-line). This could result in None values, explaining why data might not display.
Commands (e.g., relay control) flow similarly: generated in controller, sent to server (POST /commands), cached, and can be fetched (GET /commands).
No persistent storage (e.g., database/files); caching is in-memory only, so data is lost on server restart.
Logging (via casaverde_utils::init_logger) writes to logs/<binary>.log in the binary's directory, including data sends/fetches, but this is separate from caching.

Does casaverde_server read/write any of the sensor data?Yes:Read: It receives sensor data via POST /sensor_data (from controller or app) and reads from its in-memory cache for GET requests (/temps or /sensor_data).
Write: It writes received data to the in-memory temp cache (insert_temp_cache). No hardware access or file/DB writes; it's purely a cache layer to offload data handling from controller/app.
Caching uses timestamps but doesn't expire data automatically—it's always available until overwritten or server restarts.
Logs include received data (e.g., "Received POST data: client_id=..., devices=...").

Does casaverde_app read/write any of the sensor data?Yes, but indirectly via the server (no direct hardware access):Read: Fetches sensor data from server via GET /temps in update_devices(), processes it locally into device_values and temp_data, and displays in TUI.
Write: If >5 seconds since last update, it sends a subset (active sensors only) back to server via POST /sensor_data. This doesn't generate new data—it's essentially echoing fetched values, which could overwrite the cache if values differ (but in practice, it's the same data).
Toggles (via Enter key) only affect local state (states array), not writing to hardware/server.
Logs include sent data and rendering info.

Does casaverde_controller read/write any of the sensor data?Yes, this is the primary hardware-interacting binary:Read: Directly from hardware—GPIO for local CPU temp, serial for Arduino probe (and potentially others). Also fetches remote readings from server GET /temps to inform command generation.
Write: Sends commands to Arduino via serial (e.g., "ON_INT2\n" for relay2 based on probe temp). Sends aggregated sensor data to server via POST /sensor_data. No local file/DB writes beyond logs.
Note: Current Arduino code ignores most commands (e.g., "ON_INT2\n") and autonomously controls relay2 based on probe temp. It only handles "CPU_TEMP:" (not sent by controller), so serial writes for relays may not affect hardware.


