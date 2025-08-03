## Solar
- Auto-retracting blinds based on the angle of the sun and/or amount of sunlight 
## Temperature
- Thermostat control of the AC unit that monitors and adjust based off energy conservation algorithm
## Moisture
## Humidity
## Water

# Devices
## Security Cameras (SC)
- Wired, no WiFi
- Cameras on it's own internal network separated from all other networks and encrypted
- Video feed capable of being displayed on any device in the house
## Water Control Unit (H20CU)
- Controls water flow, schedule, and such for water the garden
- Monitors rain water container level, collection rate, and flow (including overflow)
## Solar Sensor Unit (SSU)
- Monitors and feeds information into algorithm that calculates every 24 hours 
- Solar data used to control auto-blinds, greenhouse blinds, and real time data on solar position
# Weather Control Unit (WCU)
- Comprised of SSU, H20CU, TS, and HCU
## Thermostat (TS)
- Internal thermostat attached to the house's AC unit
- External thermostat attached to the greenhouse
## Humidity Control Unit (HCU)
- Monitors and adjusts humidity for greenhouse
## Light Control Unit (LCU)
- Internal lights (Philips Hue) use API to link with the bridge to control lights, add motion sensors so when walking through rooms during night, lights just auto on/off as you walk through house
- External lights (FLOOD LIGHTS) because nothing says get off my grass like flood lights
# Casaverde
## Server (casaverde_server/HUB)
- Main server
## Client (casaverde_app)
- Wired into the device 
## Controller (casaverde_controller)
- Touchscreen pad
- TUI
- Android/iPhone app