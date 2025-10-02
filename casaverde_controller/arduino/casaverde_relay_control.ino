// casaverde_relay_control
const int relay1 = 2; // INT1 (Red LED, CPU cooling)
const int relay2 = 3; // INT2 (Blue LED, GPU cooling)
const int relay3 = 4; // LIGHT1 (Red LED, Light ON)
const int relay4 = 5; // LIGHT2 (Green LED, Light OFF)

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
}

void loop() {
  if (Serial.available() > 0) {
    String command = Serial.readStringUntil('\n');
    command.trim();
    if (command == "ON_INT1") {
      digitalWrite(relay1, HIGH);
      Serial.println("Red LED turned on (CPU cooling)");
    } else if (command == "OFF_INT1") {
      digitalWrite(relay1, LOW);
      Serial.println("Red LED turned off (CPU cooling)");
    } else if (command == "ON_INT2") {
      digitalWrite(relay2, HIGH);
      Serial.println("Blue LED turned on (GPU cooling)");
    } else if (command == "OFF_INT2") {
      digitalWrite(relay2, LOW);
      Serial.println("Blue LED turned off (GPU cooling)");
    } else if (command == "ON_LIGHT1") {  // New command for light ON
      digitalWrite(relay3, HIGH);
      Serial.println("Light turned ON (Red LED)");
    } else if (command == "OFF_LIGHT2") { // New command for light OFF
      digitalWrite(relay4, HIGH);
      Serial.println("Light turned OFF (Green LED)");
    } else if (command == "OPEN_VALVE1") {
      // Valve control remains as is (though no relay assigned now)
      Serial.println("Valve Open command received (no relay assigned)");
    } else if (command == "CLOSE_VALVE2") {
      Serial.println("Valve Close command received (no relay assigned)");
    } else if (command == "TEST_CYCLE") {
      // Optional: Adjust test cycle for light simulation
      digitalWrite(relay3, HIGH); // Light ON
      Serial.println("Light ON: TEST TEST TEST");
      delay(3000);
      digitalWrite(relay3, LOW);
      digitalWrite(relay4, HIGH); // Light OFF
      Serial.println("Light OFF: TEST TEST TEST");
      delay(3000);
      digitalWrite(relay4, LOW);
      Serial.println("TEST COMPLETE");
    } else {
      Serial.println("Unknown command: " + command);
    }
  }
}
