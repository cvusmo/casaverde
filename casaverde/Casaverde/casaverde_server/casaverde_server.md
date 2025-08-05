# Overview:
The [[casaverde_server]] binary is the central server component of the Casaverde home automation system, designed to run on a dedicated machine to manage sensor data collection, storage, and distribution.
# Purpose:
- Serves as the central data hub for the Casaverde system, receiving sensor data from client devices, storing it, and providing it to controllers.
- Ensures secure communication through TLS certificates, with dynamic configuration of server addresses and certificate paths to avoid hardcoding.
- Provides a foundation for future expansion including greenhouse control, AC management, and integration with external services by exposing configurable endpoints.
# Contents:
- [[cache.rs]]
- [[handlers.rs]]
- [[Casaverde/casaverde_server/main.rs/main.rs|main.rs]]
- [[models.rs]]
