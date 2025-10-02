Represents the configuration for a single device, including its identifier, type, endpoint, and polling interval.
- Fields
	- id: String - unique identifier for the device
	- r#type: String - type of device or sensor intended as placeholder for future enum-based typing
	- endpoint: String - the URL endpoint for sending sensor data to the server
	- interval: u32 - the polling interval in seconds for updating the device data.