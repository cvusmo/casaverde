A structure to serialize sensor data for posting to the server.
- Fields
	- client_id: String - id of the client sending the data
	- devices: Vec<[[Casaverde/casaverde_app/devices.rs/structs/DeviceReading]]> - A vector of [[Casaverde/casaverde_app/devices.rs/structs/DeviceReading]] instances representing the current state of all devices.