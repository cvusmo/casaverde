# Overview:
The casaverde_controller binary is a client-side component of the Casaverde home automation system, designed to run on distributed devices to fetch sensor data from the central [[casaverde_server]] and control hardware (e.g., cooling systems) based on that data.
# Purpose:
- Periodically retrieves temperature data from the [[casaverde_server]] using secure HTTPS requests with TLS certificates.
- Processes sensor data to make decisions about hardware control, such as turning cooling systems on or off based on temperature thresholds.
- Provides a foundation for future enhancements, including GPIO relay control for hardware and energy optimization algorithms.
- Ensures secure and configurable communication with the server through a TOML configuration file and certificate-based authentication.
# Contents:
- [[Casaverde/casaverde_controller/main.rs/main.rs|main.rs]]
