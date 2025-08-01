# Design Concept
## Issues
### FIX NOW
- app(client) needs to collect its own data, send it to the server to be cache'd
- multiple clients should be able to run which unique ID's to keep them independent and to help server cache data efficiently
- each device would be using it's own client to tx/rx data. for testing we're using two pc's to get various sensor data from them and then displaying it on the main display.
- do we now create a dedicated binary for the controller? casaverde_controller?