The root configuration structure loaded from [[config.toml]], containing the server URL and a list of device configurations.

- Fields
	- server: String - the base URL of the [[casaverde_server]]
	- configs: Vec<[[DeviceConfig]] - a vector of [[DeviceConfig]] instances defining all monitored devices