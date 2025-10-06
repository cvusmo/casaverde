#include <OneWire.h>
#include <DallasTemperature.h>

// Pin definitions
#define ONE_WIRE_BUS 6  // DS18B20 data pin
#define RELAY_1 2       // Relay 1 control pin
#define RELAY_2 3       // Relay 2 control pin
#define RELAY_3 4       // Relay 3 control pin (for future PCB)
#define RELAY_4 5       // Relay 4 control pin (for future PCB)

// Setup OneWire and DallasTemperature libraries
OneWire oneWire(ONE_WIRE_BUS);
DallasTemperature sensors(&oneWire);

void setup() {
  // Initialize serial communication at 9600 baud
  Serial.begin(9600);
  
  // Initialize relay pins as outputs
  pinMode(RELAY_1, OUTPUT);
  pinMode(RELAY_2, OUTPUT);
  pinMode(RELAY_3, OUTPUT);
  pinMode(RELAY_4, OUTPUT);
  
  // Set relays to OFF (active-low relays)
  digitalWrite(RELAY_1, HIGH);
  digitalWrite(RELAY_2, HIGH);
  digitalWrite(RELAY_3, HIGH);
  digitalWrite(RELAY_4, HIGH);
  
  // Initialize DS18B20 sensor
  sensors.begin();
}

void loop() {
  // Check for incoming serial commands with timeout
  if (Serial.available() > 0) {
    String command = "";
    unsigned long startTime = millis();
    while (millis() - startTime < 1000) { // 1-second timeout
      if (Serial.available() > 0) {
        char c = Serial.read();
        if (c == '\n') break;
        command += c;
      }
    }
    command.trim(); // Remove whitespace

    // Handle temperature probe request
    if (command == "GET probe-temp") {
      sensors.requestTemperatures();
      float tempC = sensors.getTempCByIndex(0);
      if (tempC != DEVICE_DISCONNECTED_C) {
        Serial.print("TEMP_PROBE:");
        Serial.print(tempC);
        Serial.println();
      } else {
        Serial.println("TEMP_PROBE:ERROR");
      }
    }
    // Handle relay control commands
    else if (command == "SET relay-1 ON") {
      digitalWrite(RELAY_1, LOW); // Active-low relay
      Serial.println("relay-1:ON");
    }
    else if (command == "SET relay-1 OFF") {
      digitalWrite(RELAY_1, HIGH);
      Serial.println("relay-1:OFF");
    }
    else if (command == "SET relay-2 ON") {
      digitalWrite(RELAY_2, LOW);
      Serial.println("relay-2:ON");
    }
    else if (command == "SET relay-2 OFF") {
      digitalWrite(RELAY_2, HIGH);
      Serial.println("relay-2:OFF");
    }
    else if (command == "SET relay-3 ON") {
      digitalWrite(RELAY_3, LOW);
      Serial.println("relay-3:ON");
    }
    else if (command == "SET relay-3 OFF") {
      digitalWrite(RELAY_3, HIGH);
      Serial.println("relay-3:OFF");
    }
    else if (command == "SET relay-4 ON") {
      digitalWrite(RELAY_4, LOW);
      Serial.println("relay-4:ON");
    }
    else if (command == "SET relay-4 OFF") {
      digitalWrite(RELAY_4, HIGH);
      Serial.println("relay-4:OFF");
    }
    else {
      Serial.println("ACK");
    }
  }
}
