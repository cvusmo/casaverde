#include <OneWire.h>
#include <DallasTemperature.h>

#define ONE_WIRE_BUS 6
OneWire oneWire(ONE_WIRE_BUS);
DallasTemperature sensors(&oneWire);

const int relay1 = 2; // Red LED (manual)
const int relay2 = 3; // Blue LED (temperature probe)
const int relay3 = 4; // Green LED (manual)
const int relay4 = 5; // Yellow LED (CPU temp)

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
  sensors.requestTemperatures();
  float probeTemp = sensors.getTempCByIndex(0);

  // Temperature probe controls relay2
  if (probeTemp != DEVICE_DISCONNECTED_C) {
    if (probeTemp > 26.0) {
      digitalWrite(relay2, HIGH); // LED on
    } else {
      digitalWrite(relay2, LOW); // LED off
    }
    Serial.print("TEMP_PROBE:");
    Serial.println(probeTemp);
  } else {
    Serial.println("TEMP_PROBE:ERROR");
  }

  // CPU temperature comes from host (Rust side) via command
  if (Serial.available() > 0) {
    String cmd = Serial.readStringUntil('\n');
    cmd.trim();
    if (cmd.startsWith("CPU_TEMP:")) {
      float cpuTemp = cmd.substring(9).toFloat();
      if (cpuTemp > 40.0) {
        digitalWrite(relay4, HIGH);
      } else {
        digitalWrite(relay4, LOW);
      }
    }
  }

  delay(1000); // 1 second loop
}

