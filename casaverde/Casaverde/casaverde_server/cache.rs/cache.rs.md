# Purpose:
- Provides a thread-safe caching mechanism to store sensor data [[Casaverde/casaverde_app/devices.rs/structs/SensorReading]] from clients
- Enables the server to maintain a persistent state of device readings, facilitating real-time access by endpoints and future controller interactions
## Contents:
### Methods:
- [[get_cache]]
- [[insert_cache]]
