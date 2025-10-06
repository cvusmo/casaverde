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
  // Probe temperature sensor
  sensors.requestTemperatures();
  float probeTemp = sensors.getTempCByIndex(0);
  if (probeTemp != DEVICE_DISCONNECTED_C) {
    Serial.print("TEMP_PROBE:");
    Serial.println(probeTemp);
  } else {
    Serial.println("TEMP_PROBE:ERROR");
  }

  // CPU temperature / relay commands from controller
  if (Serial.available() > 0) {
    String cmd = Serial.readStringUntil('\n');
    cmd.trim();

    if (cmd == "SET RELAY1 ON") digitalWrite(relay1, HIGH);
    else if (cmd == "SET RELAY1 OFF") digitalWrite(relay1, LOW);
    else if (cmd == "SET RELAY2 ON") digitalWrite(relay2, HIGH);
    else if (cmd == "SET RELAY2 OFF") digitalWrite(relay2, LOW);
    else if (cmd == "SET RELAY3 ON") digitalWrite(relay3, HIGH);
    else if (cmd == "SET RELAY3 OFF") digitalWrite(relay3, LOW);
    else if (cmd == "SET RELAY4 ON") digitalWrite(relay4, HIGH);
    else if (cmd == "SET RELAY4 OFF") digitalWrite(relay4, LOW);
  }

  delay(1000);
}

