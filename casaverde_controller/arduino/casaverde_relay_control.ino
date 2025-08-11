// casaverde_relay_control
const int relay1 = 2; // INT1 (Red LED)
const int relay2 = 3; // INT2 (Blue LED)
const int relay3 = 4; // VALVE1 (Yellow LED)
const int relay4 = 5; // VALVE2 (Green LED)

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
      Serial.println("Red LED turned on");
    } else if (command == "OFF_INT1") {
      digitalWrite(relay1, LOW);
      Serial.println("Red LED turned off");
    } else if (command == "ON_INT2") {
      digitalWrite(relay2, HIGH);
      Serial.println("Blue LED turned on");
    } else if (command == "OFF_INT2") {
      digitalWrite(relay2, LOW);
      Serial.println("Blue LED turned off");
    } else if (command == "OPEN_VALVE1") {
      digitalWrite(relay4, LOW);
      digitalWrite(relay3, HIGH);
      delay(30);
      digitalWrite(relay3, LOW);
      Serial.println("Solenoid Open: Yellow LED");
    } else if (command == "CLOSE_VALVE2") {
      digitalWrite(relay3, LOW);
      digitalWrite(relay4, HIGH);
      delay(30);
      digitalWrite(relay4, LOW);
      Serial.println("Solenoid Closed: Green LED");
    } else if (command == "TEST_CYCLE") {
      digitalWrite(relay4, LOW);
      digitalWrite(relay3, HIGH);
      delay(30); // OPEN VALVE
      digitalWrite(relay3, LOW);
      Serial.println("Solenoid OPEN: TEST TEST TEST");
      delay(10000);
      digitalWrite(relay3, LOW);
      digitalWrite(relay4, HIGH);
      delay(30); // CLOSE VALVE
      digitalWrite(relay4, LOW);
      Serial.println("Solenoid CLOSED: TEST TEST TEST");
      Serial.println("TEST COMPLETE");
    } else {
      Serial.println("Unknown command: " + command);
    }
  }
}
