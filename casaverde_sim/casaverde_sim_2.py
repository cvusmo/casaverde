# casaverde_sim_2.py
#!/usr/bin/env python3
import serial
import subprocess
import random
import re
import time
from datetime import datetime, timedelta

ser = serial.Serial("/tmp/virtualcom1", baudrate=9600, timeout=1)
relay_state = "OFF"
fan_state = "OFF"
water_valve_open = False
solar_state = "ON"  # Default to ON for 18h cycle
humidity = 70.0  # Starting humidity
last_humidity_update = time.time()
probe_temp = random.uniform(10.0, 20.0)  # Starting probe temp
last_probe_update = time.time()
last_water_flow = time.time()
water_flow_times = [8, 16, 24]  # 8 AM, 4 PM, midnight (in 24h format)

print("Simulator started. Waiting for commands...")


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
                response = f"SOLAR:{random.uniform(50.0, 150.0) if solar_state == 'ON' else 0.0:.1f}W\n"
            elif "blackbeard-probe" in received.lower():
                current_time = time.time()
                if current_time - last_probe_update >= 60:
                    probe_temp += random.uniform(-2.0, 2.0)
                    probe_temp = max(5.0, min(25.0, probe_temp))
                    last_probe_update = current_time
                response = f"TEMP_PROBE:{probe_temp:.1f}\n"
            elif "moisture-1" in received.lower():
                response = f"MOISTURE:{random.uniform(0.0, 100.0):.1f}%\n"
            elif "humidity-1" in received.lower():
                current_time = time.time()
                if current_time - last_humidity_update >= 60:  # Update every 60s
                    humidity += random.uniform(-5.0, 5.0)  # ±5% variation
                    humidity = max(
                        20.0, min(90.0, humidity)
                    )  # Keep within realistic range
                    last_humidity_update = current_time
                response = f"HUMIDITY:{humidity:.1f}%\n"
            elif "water-1" in received.lower():
                current_time = time.time()
                current_hour = datetime.fromtimestamp(current_time).hour
                if not water_valve_open and any(
                    abs(current_hour - t) < 1 for t in water_flow_times
                ):  # 3x daily
                    water_valve_open = True
                    last_water_flow = current_time
                elif (
                    water_valve_open and current_time - last_water_flow >= 300
                ):  # Flow for 5 mins
                    water_valve_open = False
                response = f"WATER:{random.uniform(0.0, 100.0) if water_valve_open else 0.0:.1f}\n"
            elif "relay-1" in received.lower() or "light1" in received.lower():
                if "SET relay-1 ON" in received or "ON_LIGHT1" in received:
                    relay_state = "ON"
                    response = "RELAY:ON\n"
                elif "SET relay-1 OFF" in received or "OFF_LIGHT1" in received:
                    relay_state = "OFF"
                    response = "RELAY:OFF\n"
                else:
                    response = f"RELAY:{relay_state}\n"
            elif "fan1" in received.lower():
                if "SET FAN1 ON" in received:
                    fan_state = "ON"
                    response = "FAN:ON\n"
                elif "SET FAN1 OFF" in received:
                    fan_state = "OFF"
                    response = "FAN:OFF\n"
                else:
                    response = f"FAN:{fan_state}\n"
            else:
                response = "ACK\n"
            ser.write(response.encode("utf-8"))
            print(f"Sent response: {response.strip()}")

        # Update solar state based on 18h ON (6 AM - midnight), 6h OFF (midnight - 6 AM)
        current_hour = datetime.now().hour
        solar_state = "ON" if 6 <= current_hour < 24 else "OFF"

    except Exception as e:
        print(f"Error: {e}")
        break
