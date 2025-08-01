# Purpose:
- Defines the [[Sensor]] enum and [[Casaverde/casaverde_app/app.rs/SensorData]] struct to manage sensor states and fetch multiple sensor data from the casaverde_server. It follows Data-Oriented Design (DOD) by organizing data in a cache-friendly manner and Data-Oriented Programming by treating data as immutable where possible and minimizing side effects in methods.
## Contents:
### Structs:
- [[TempData]]
- [[Casaverde/casaverde_app/sensors.rs/structs/SensorData|SensorData]]
### Enums:
- [[Sensor]]
	- Variants:
		- [[Solar]]
		- [[Temperature]]
		- [[Moisture]]
		- [[Humidity]]
		- [[Water]]
		- #TODO add more
### Methods:
- [[Casaverde/casaverde_app/sensors.rs/methods/new|new]]
- [[update_temperatures]]
- [[toggle_sensor]]