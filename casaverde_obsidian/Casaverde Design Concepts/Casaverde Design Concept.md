# Design Concept
## 74HC595
- VCC (PIN16) -> +5V
-  GND (PIN8) -> GND
- DS (PIN14) -> D12
- ST_CP (PIN12) -> D11
- SH_CP (PIN11) -> D10
- OE (PIN13) -> GND
- MR (PIN9) -> 5V
- Q0 (PIN15) -> Relay IN1

## DS18B20 Temperature Sensor
- VCC -> +5V
- GND -> GND
- DATA -> D2 (4.7k)

## 4-Channel Relay
- VCC -> +5V
- GND -> GND
- IN1 -> Q0 
- IN2, IN3, IN4 NOT CONNECTED
- COM -> +5V