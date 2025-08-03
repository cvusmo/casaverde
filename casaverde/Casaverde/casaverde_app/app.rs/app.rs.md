# Purpose:
- Defines the [[CasaverdeApp]] struct and methods for managing application state, navigation, quitting, and switching screens.
- Serves as the core state management module for the [[casaverde_app]], integrates sensor data updates from the [[DeviceData]] module and rendering the TUI interface.
- Provides a flexible framework for monitoring various sensors and displaying their status, with potential future expansion to other devices.
## Contains:
- [[Casaverde/casaverde_app/app.rs/enums/Screen|Screen]] enum
	- Represents the different views or screens in the TUI
	- Variants
		- Sensors: Displays a list of sensors for selection and toggling
		- Monitoring: Shows real-time temperature monitoring data
		- #TODO 
			- add mainmenu
			- add settings
			- add cameraview
- [[CasaverdeApp]] struct
	- Holds the application state, coordinating sensor data and user interface interactions.
	- Fields
		- [[sensor_data]]: 
			- [[DeviceData]] instance managing sensor states and temperature data.
		- [[selected]]:
			- usize index of the currently selected sensor in the [[Sensors]] screen.
		- [[should_quit]]:
			- bool flag to indicate when to exit the application
		- [[screen]]
			- an instance of the [[Screen]] enum

## Methods:
- [[Casaverde/casaverde_app/app.rs/methods/new/new|new]]
- [[move_up]]
- [[move_down]]
- [[quit]]
- [[switch_screen]]
- [[run_app]]