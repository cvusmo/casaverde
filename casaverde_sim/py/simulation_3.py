#!/usr/bin/env python3
import serial
import subprocess
import random
import time
import json
import logging
from datetime import datetime, timedelta
from pathlib import Path

# --- Logging setup ---
logging.basicConfig(
    filename="/home/echo/casaverde/target/build/log/python_sim.log",
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S",
)

# --- Serial setup ---
try:
    ser = serial.Serial("/tmp/virtualcom1", baudrate=9600, timeout=1)
except Exception as e:
    logging.error(f"Failed to open serial port: {e}")
    raise

# --- Simulation state ---
relay_state = {"relay-1": "OFF", "relay-2": "OFF", "relay-3": "OFF", "relay-4": "OFF"}
fan_state = "OFF"
water_valve_open = False
solar_state = "ON"
humidity = 70.0
probe_temp = 15.0
moisture = 50.0  # Initialize from casaverde_sim if available
nutrients = 0.5  # New sensor from casaverde_sim
sim_time = datetime(2025, 1, 1, 6, 0, 0)
SIM_STEP = timedelta(seconds=10)  # Each loop = 10 seconds

# Water/solar schedule
water_flow_times = [8, 16, 24]  # hours
solar_on_hours = range(6, 24)

# Path for casaverde_sim output
SIM_DATA_FILE = "/tmp/casaverde_sim_data.json"

logging.info("Simulator started. Waiting for commands...")


def get_real_cpu_temp():
    try:
        result = subprocess.run(["sensors"], capture_output=True, text=True)
        output = result.stdout
        for line in output.splitlines():
            if "Package id 0:" in line:
                temp_str = line.split("+")[1].split("°")[0]
                logging.info(f"Read CPU temp: {temp_str}")
                return float(temp_str)
    except Exception as e:
        logging.error(f"Error fetching CPU temp: {e}")
    return None


def read_sim_data():
    global moisture, nutrients
    try:
        if Path(SIM_DATA_FILE).exists():
            with open(SIM_DATA_FILE, "r") as f:
                data = json.load(f)
                if (
                    data
                    and isinstance(data, list)
                    and "moisture" in data[0]
                    and "nutrients" in data[0]
                ):
                    moisture = data[0]["moisture"] * 100.0  # Convert 0.0-1.0 to 0-100%
                    nutrients = data[0]["nutrients"]  # 0.0-1.0
                    logging.info(
                        f"Read sim data: moisture={moisture:.1f}%, nutrients={nutrients:.3f}"
                    )
    except Exception as e:
        logging.error(f"Error reading sim data: {e}")


while True:
    try:
        # --- Read casaverde_sim data ---
        read_sim_data()

        # --- Read commands from controller ---
        received = ser.readline().decode("utf-8").strip()
        if received:
            logging.info(f"Received from controller: {received}")
            print(f"Received from controller: {received}")

            # CPU temp request
            if "blackbeard-cpu" in received.lower():
                cpu_temp = get_real_cpu_temp()
                response = (
                    f"TEMP:{cpu_temp:.2f}\n" if cpu_temp is not None else "TEMP:ERROR\n"
                )

            # Solar sensor
            elif "solar-1" in received.lower():
                response = f"SOLAR:{random.uniform(50.0, 150.0) if solar_state == 'ON' else 0.0:.1f}W\n"

            # Probe temperature
            elif "blackbeard-probe" in received.lower():
                response = f"TEMP_PROBE:{probe_temp:.1f}\n"

            # Moisture sensor (from casaverde_sim)
            elif "moisture-1" in received.lower():
                response = f"MOISTURE:{moisture:.1f}%\n"

            # Nutrients sensor (new, from casaverde_sim)
            elif "nutrients-1" in received.lower():
                response = f"NUTRIENTS:{nutrients:.3f}\n"

            # Humidity sensor
            elif "humidity-1" in received.lower():
                response = f"HUMIDITY:{humidity:.1f}%\n"

            # Water sensor
            elif "water-1" in received.lower():
                response = f"WATER:{random.uniform(0.0, 100.0) if water_valve_open else 0.0:.1f}\n"

            # Relay control
            elif "relay" in received.lower() or "light" in received.lower():
                for r in relay_state.keys():
                    if f"SET {r} ON" in received:
                        relay_state[r] = "ON"
                    elif f"SET {r} OFF" in received:
                        relay_state[r] = "OFF"
                response = ",".join([f"{k}:{v}" for k, v in relay_state.items()]) + "\n"

            # Fan control
            elif "fan1" in received.lower():
                if "SET FAN1 ON" in received:
                    fan_state = "ON"
                elif "SET FAN1 ON" in received:
                    fan_state = "OFF"
                response = f"FAN:{fan_state}\n"

            else:
                response = "ACK\n"

            ser.write(response.encode("utf-8"))
            logging.info(f"Sent response: {response.strip()}")
            print(f"Sent response: {response.strip()}")

        # --- Simulation clock advance ---
        sim_time += SIM_STEP

        # --- Update simulated sensors ---
        probe_temp += random.gauss(0, 0.2)
        probe_temp = max(5.0, min(25.0, probe_temp))

        humidity += random.uniform(-0.5, 0.5)
        humidity = max(20.0, min(90.0, humidity))

        # Water valve schedule
        if not water_valve_open and sim_time.hour in water_flow_times:
            water_valve_open = True
            water_open_time = sim_time
        elif water_valve_open and (sim_time - water_open_time).seconds >= 300:
            water_valve_open = False

        # Solar on/off
        solar_state = "ON" if sim_time.hour in solar_on_hours else "OFF"

        # --- Print and log all simulated sensor values ---
        status = (
            f"[{sim_time.strftime('%H:%M:%S')}] Probe:{probe_temp:.1f}°C Humidity:{humidity:.1f}% "
            f"Water:{'OPEN' if water_valve_open else 'CLOSED'} Solar:{solar_state} "
            f"Moisture:{moisture:.1f}% Nutrients:{nutrients:.3f} Relays:{relay_state}"
        )
        print(status)
        logging.info(status)

        time.sleep(0.1)  # fast simulation loop

    except Exception as e:
        logging.error(f"Simulator error: {e}")
        print(f"Simulator error: {e}")
        break
