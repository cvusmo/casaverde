#include <OneWire.h>
#include <DallasTemperature.h>

// ---------------------------
// Pin Definitions
// ---------------------------
#define ONE_WIRE_BUS 6
const int relayPins[] = {2, 3, 4, 5};
const int numRelays = sizeof(relayPins) / sizeof(relayPins[0]);
const int fanPin = 7; // example fan pin

// ---------------------------
// OneWire & Temperature
// ---------------------------
OneWire oneWire(ONE_WIRE_BUS);
DallasTemperature sensors(&oneWire);

// ---------------------------
// Serial Settings
// ---------------------------
const unsigned long SERIAL_BAUD = 9600;
const unsigned long LOOP_DELAY_MS = 100; // match simulation speed

// ---------------------------
// Simulation State
// ---------------------------
bool fanState = false;
bool waterValveOpen = false;
bool solarState = true; // ON
float humidity = 70.0;
float probeTemp = 15.0;
String relayState[4] = {"OFF", "OFF", "OFF", "OFF"};

// ---------------------------
// Helper Functions
// ---------------------------
void sendProbeTemperature() {
  sensors.requestTemperatures();
  float temp = sensors.getTempCByIndex(0);

  if (temp != DEVICE_DISCONNECTED_C) {
    Serial.print("TEMP_PROBE:");
    Serial.println(temp, 1); // 1 decimal place
  } else {
    Serial.println("TEMP_PROBE:ERROR");
  }
}

void sendHumidity() {
  Serial.print("HUMIDITY:");
  Serial.print(humidity, 1);
  Serial.println("%");
}

void sendWater() {
  Serial.print("WATER:");
  Serial.println(waterValveOpen ? "OPEN" : "CLOSED");
}

void sendSolar() {
  Serial.print("SOLAR:");
  Serial.println(solarState ? "120.0" : "0.0"); // placeholder W
}

void sendFanState() {
  Serial.print("FAN:");
  Serial.println(fanState ? "ON" : "OFF");
}

void handleSerialCommands() {
  while (Serial.available() > 0) {
    String cmd = Serial.readStringUntil('\n');
    cmd.trim();

    // normalize command: uppercase and remove hyphens
    cmd.toUpperCase();
    cmd.replace("-", "");

    // Relay commands
    for (int i = 0; i < numRelays; i++) {
      String onCmd = "SET RELAY" + String(i + 1) + " ON";
      String offCmd = "SET RELAY" + String(i + 1) + " OFF";

      if (cmd.equals(onCmd)) {
        digitalWrite(relayPins[i], HIGH);
        relayState[i] = "ON";
        Serial.println("ACK:" + onCmd);
      } else if (cmd.equals(offCmd)) {
        digitalWrite(relayPins[i], LOW);
        relayState[i] = "OFF";
        Serial.println("ACK:" + offCmd);
      }
    }

    // Fan commands
    if (cmd.equals("SET FAN1 ON")) {
      digitalWrite(fanPin, HIGH);
      fanState = true;
      Serial.println("ACK:SET FAN1 ON");
    } else if (cmd.equals("SET FAN1 OFF")) {
      digitalWrite(fanPin, LOW);
      fanState = false;
      Serial.println("ACK:SET FAN1 OFF");
    }

    // Sensor queries
    if (cmd.equals("BLACKBEARDPROBE") || cmd.equals("BLACKBEARDCPU")) {
      sendProbeTemperature(); // CPU can be same as probe for placeholder
    } else if (cmd.equals("HUMIDITY1")) {
      sendHumidity();
    } else if (cmd.equals("WATER1")) {
      sendWater();
    } else if (cmd.equals("SOLAR1")) {
      sendSolar();
    }

    // Send relay and fan state on generic ACK
    if (cmd.equals("STATUS")) {
      String status = "";
      for (int i = 0; i < numRelays; i++) {
        status += "RELAY" + String(i + 1) + ":" + relayState[i] + ",";
      }
      status += "FAN:" + String(fanState ? "ON" : "OFF");
      Serial.println(status);
    }
  }
}

// ---------------------------
// Setup
// ---------------------------
void setup() {
  Serial.begin(SERIAL_BAUD);
  while (!Serial); // wait for serial connection (optional)

  // Relays
  for (int i = 0; i < numRelays; i++) {
    pinMode(relayPins[i], OUTPUT);
    digitalWrite(relayPins[i], LOW);
  }

  // Fan
  pinMode(fanPin, OUTPUT);
  digitalWrite(fanPin, LOW);

  // Temperature sensor
  sensors.begin();

  Serial.println("Arduino initialized");
}

// ---------------------------
// Loop
// ---------------------------
void loop() {
  // Update simulated sensors
  probeTemp += random(-2, 3) * 0.1; // small random change
  probeTemp = constrain(probeTemp, 5.0, 25.0);

  humidity += random(-5, 6) * 0.1;
  humidity = constrain(humidity, 20.0, 90.0);

  // Water valve / solar placeholders (simulate schedule if desired)
  // Example: toggle water every 10 loops
  static int loopCount = 0;
  if (loopCount % 10 == 0) waterValveOpen = !waterValveOpen;
  solarState = !solarState; // toggle solar for demo

  // Send probe temp each loop
  sendProbeTemperature();

  // Handle serial commands
  handleSerialCommands();

  loopCount++;
  delay(LOOP_DELAY_MS);
}
