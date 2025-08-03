# Purpose:
- Defines the [[CasaverdeApp]] struct and methods for state management (navigation, quitting, screen switching)
- Serves as the core state management module for the [[casaverde_app]], coordinating the TUI interface and sensor data updates.
## Contains:
- [[Casaverde/casaverde_app/app.rs/enums/Screen|Screen]] enum
	- Represents the different views or screens in the TUI
	- Variants
		- Sensors: Displays a list of sensors for selection and toggling
		- Monitoring: Shows real-time temperature monitoring data
- [[CasaverdeApp]] struct
	- Holds the application state, including sensor data, selected item, quit flag, and current screen.
	- Fields
		- [[sensor_data]]: 
			- [[Casaverde/casaverde_app/app.rs/SensorData]] instance managing sensor states and temperature data.
		- [[selected]]:
			- usize index of the currently selected sensor in the [[Sensors]] screen.
		- [[should_quit]]:
			- bool flag to indicate when to exit the application
		- [[Casaverde/casaverde_app/app.rs/enums/Screen]]
			- [[Casaverde/casaverde_app/app.rs/enums/Screen]] enum indicating the current view

## Methods:
- [[Casaverde/casaverde_app/app.rs/methods/new/new]]
- [[move_up]]
- [[move_down]]
- [[quit]]
- [[switch_screen]]
- [[run_app]]