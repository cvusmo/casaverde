// Uno R3 Sketch
const int relay1 = 2; // INT1 (Red LED)
const int relay2 = 3; // INT2 (Blue LED)

void setup() {
  Serial.begin(9600); // Match RPi5 baud rate
  pinMode(relay1, OUTPUT);
  pinMode(relay2, OUTPUT);
  digitalWrite(relay1, LOW);
  digitalWrite(relay2, LOW);
}

void loop() {
  if (Serial.available() > 0) {
    String command = Serial.readStringUntil('\n');
    command.trim();
    if (command == "ON_INT1") digitalWrite(relay1, HIGH);
    else if (command == "OFF_INT1") digitalWrite(relay1, LOW);
    else if (command == "ON_INT2") digitalWrite(relay2, HIGH);
    else if (command == "OFF_INT2") digitalWrite(relay2, LOW);
  }
}
