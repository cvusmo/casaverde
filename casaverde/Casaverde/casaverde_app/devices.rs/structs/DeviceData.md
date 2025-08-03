The primary structure managing device states, values, and communication with the server. 
- Fields
	- device_values: Vec<Option<[[f32]]>> - Vector storing values for each configured devices, with None indicating no data
	- last_updated: Instant - timestamp for last device data update
	- client: [[Client]] - instance configured with TLS for secure HTTP requests using crate reqwest
	- config: [[DeviceConfigRoot]] - loaded configuration structure
	- active_count: usize - the number of active devices capped at 16 for performance (testing will expand)