# Purpose:
- Defines the [[DeviceConfig]] enum and [[DeviceData]] struct to manage sensor states and fetch multiple sensor data from the casaverde_server. It follows Data-Oriented Design (DOD) by organizing data in a cache-friendly manner and Data-Oriented Programming by treating data as immutable where possible and minimizing side effects in methods.
## Contents:
### Structs:
- [[DeviceConfig]]
- [[DeviceConfigRoot]]
- [[Casaverde/casaverde_app/devices.rs/structs/SensorReading]]
- [[Casaverde/casaverde_app/devices.rs/structs/DeviceReading]]
- [[DeviceData]]
### Methods:
- [[Casaverde/casaverde_app/devices.rs/methods/new|new]]
- [[update_devices]]