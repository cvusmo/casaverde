#include <OneWire.h>
#include <DallasTemperature.h>
#define ONE_WIRE_BUS 6
OneWire oneWire(ONE_WIRE_BUS);
DallasTemperature sensors(&oneWire);
const int relay1 = 2;
const int relay2 = 3;
const int relay3 = 4;
const int relay4 = 5;

void setup() {
  Serial.begin(9600);
  pinMode(relay1, OUTPUT);
  pinMode(relay2, OUTPUT);
  pinMode(relay3, OUTPUT);
  pinMode(relay4, OUTPUT);
  digitalWrite(relay1, LOW);
  digitalWrite(relay2, LOW);
  digitalWrite(relay3, LOW);
  digitalWrite(relay4, LOW);
  sensors.begin();
  Serial.println("Arduino started");
}

void loop() {
  static unsigned long lastPrint = 0;
  if (millis() - lastPrint >= 5000) {
    Serial.println("Arduino running");
    lastPrint = millis();
  }
  if (Serial.available() > 0) {
    String command = Serial.readStringUntil('\n');
    command.trim();
    if (command == "GET_TEMP_PROBE") {
      sensors.requestTemperatures();
      float tempC = sensors.getTempCByIndex(0);
      if (tempC != DEVICE_DISCONNECTED_C) {
        Serial.print("TEMP_PROBE:");
        Serial.println(tempC);
      } else {
        Serial.println("TEMP_PROBE:ERROR");
      }
    }
    if (command == "ON_INT1") {
      digitalWrite(relay1, HIGH);
      Serial.println("Red LED turned on");
    } else if (command == "OFF_INT1") {
      digitalWrite(relay1, LOW);
      Serial.println("Red LED turned off");
    }
    if (command == "ON_INT2") {
      digitalWrite(relay2, HIGH);
      Serial.println("Blue LED turned on");
    } else if (command == "OFF_INT2") {
      digitalWrite(relay2, LOW);
      Serial.println("Blue LED turned off");
    }
    if (command == "ON_INT3") {
      digitalWrite(relay3, HIGH);
      Serial.println("Green LED turned ON");
    } else if (command == "OFF_INT3") {
      digitalWrite(relay3, LOW);
      Serial.println("Green LED turned OFF");
    }
    if (command == "ON_INT4") {
      digitalWrite(relay4, HIGH);
      Serial.println("Yellow LED turned ON");
    } else if (command == "OFF_INT4") {
      digitalWrite(relay4, LOW);
      Serial.println("Yellow LED turned OFF");
    }
  }
}
