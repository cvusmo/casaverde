#!/usr/bin/env python3
import serial
import subprocess
import random
import re


def get_real_cpu_temp():
    try:
        result = subprocess.run(["sensors"], capture_output=True, text=True)
        output = result.stdout
        cpu_match = re.search(r"Package id 0:\s*\+\d+\.\d+°C", output)
        if cpu_match:
            temp_str = re.search(r"\+\d+\.\d+°C", cpu_match.group(0)).group(0)
            return float(temp_str.strip("+°C"))
        return None
    except Exception as e:
        print(f"Error fetching CPU temp: {e}")
        return None


ser = serial.Serial("/tmp/virtualcom1", baudrate=9600, timeout=1)
relay_state = "OFF"
fan_state = "OFF"
print("Simulator started. Waiting for commands...")

while True:
    try:
        received = ser.readline().decode("utf-8").strip()
        if received:
            print(f"Received from app: {received}")
            if "blackbeard-cpu" in received.lower():
                cpu_temp = get_real_cpu_temp()
                response = (
                    f"TEMP:{cpu_temp:.2f}\n" if cpu_temp is not None else "TEMP:ERROR\n"
                )
            elif "solar-1" in received.lower():
                simulated_solar_power = random.uniform(50.0, 150.0)
                response = f"SOLAR:{simulated_solar_power:.1f}W\n"
            elif "moisture-1" in received.lower():
                simulated_moisture = random.uniform(0.0, 100.0)
                response = f"MOISTURE:{simulated_moisture:.1f}\n"
            elif "humidity-1" in received.lower():
                simulated_humidity = random.uniform(20.0, 80.0)
                response = f"HUMIDITY:{simulated_humidity:.1f}%\n"
            elif "water-1" in received.lower():
                simulated_water_level = random.uniform(0.0, 100.0)
                response = f"WATER:{simulated_water_level:.1f}\n"
            elif "relay-1" in received.lower() or "light1" in received.lower():
                if "SET relay-1 ON" in received or "ON_LIGHT1" in received:
                    relay_state = "ON"
                    response = "RELAY:OK\n"
                elif "SET relay-1 OFF" in received or "OFF_LIGHT1" in received:
                    relay_state = "OFF"
                    response = "RELAY:OK\n"
                else:
                    response = f"RELAY:{relay_state}\n"
            elif "fan1" in received.lower():
                if "SET FAN1 ON" in received:
                    fan_state = "ON"
                    response = "FAN:OK\n"
                elif "SET FAN1 OFF" in received:
                    fan_state = "OFF"
                    response = "FAN:OK\n"
                else:
                    response = f"FAN:{fan_state}\n"
            else:
                response = "ACK\n"
            ser.write(response.encode("utf-8"))
            print(f"Sent response: {response.strip()}")
    except Exception as e:
        print(f"Error: {e}")
