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

# --- Serial setup (virtual port for simulation, Arduino for real temp) ---
try:
    sim_serial = serial.Serial("/tmp/virtualcom1", baudrate=9600, timeout=1)
    arduino_serial = serial.Serial("/dev/ttyACM0", baudrate=9600, timeout=1)
except Exception as e:
    logging.error(f"Failed to open serial port: {e}")
    raise

# --- Simulation state ---
relay_state = {"relay-1": "OFF", "relay-2": "OFF", "relay-3": "OFF", "relay-4": "OFF"}
fan_state = "OFF"
water_valve_open = False
solar_state = "ON"
humidity = 70.0
probe_temp_1 = 15.0  # Simulated fallback if Arduino fails
probe_temp_2 = 15.0  # Simulated fallback for second probe
moisture = 50.0  # From casaverde_sim
nutrients = 0.5  # From casaverde_sim
sim_time = datetime(2025, 1, 1, 6, 0, 0)
SIM_STEP = timedelta(seconds=10)

# Water/solar schedule
water_flow_times = [8, 16, 24]
solar_on_hours = range(6, 24)

# Path for casaverde_sim output
SIM_DATA_FILE = "/tmp/casaverde_sim_data.json"

logging.info("Simulator started. Waiting for commands...")


def read_arduino_temps():
    """Read two temperature probe values from Arduino on /dev/ttyACM0."""
    try:
        arduino_serial.write(b"TEMP_PROBE_REQUEST\n")
        response = arduino_serial.readline().decode("utf-8").strip()
        if response.startswith("TEMP_PROBES:"):
            temps = response.split(":")[1].split(",")
            if len(temps) == 2:
                temp1, temp2 = map(float, temps)
                logging.info(
                    f"Read Arduino temps: probe1={temp1:.1f}°C, probe2={temp2:.1f}°C"
                )
                return temp1, temp2
    except Exception as e:
        logging.error(f"Error reading Arduino temps: {e}")
    return None, None


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
                    moisture = data[0]["moisture"] * 100.0
                    nutrients = data[0]["nutrients"]
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
        received = sim_serial.readline().decode("utf-8").strip()
        if received:
            logging.info(f"Received from controller: {received}")
            print(f"Received from controller: {received}")

            # Probe temperature (real data from Arduino)
            if "blackbeard-probe" in received.lower():
                temp1, temp2 = read_arduino_temps()
                if temp1 is not None:
                    response = f"TEMP_PROBE:{temp1:.1f}\n"
                else:
                    response = f"TEMP_PROBE:{probe_temp_1:.1f}\n"  # Fallback
                    probe_temp_1 += random.gauss(0, 0.2)
                    probe_temp_1 = max(5.0, min(25.0, probe_temp_1))

            # Second probe (using a different identifier, e.g., blackbeard-probe2)
            elif "blackbeard-probe2" in received.lower():
                temp1, temp2 = read_arduino_temps()
                if temp2 is not None:
                    response = f"TEMP_PROBE2:{temp2:.1f}\n"
                else:
                    response = f"TEMP_PROBE2:{probe_temp_2:.1f}\n"  # Fallback
                    probe_temp_2 += random.gauss(0, 0.2)
                    probe_temp_2 = max(5.0, min(25.0, probe_temp_2))

            # Solar sensor
            elif "solar-1" in received.lower():
                response = f"SOLAR:{random.uniform(50.0, 150.0) if solar_state == 'ON' else 0.0:.1f}W\n"

            # Moisture sensor (from casaverde_sim)
            elif "moisture-1" in received.lower():
                response = f"MOISTURE:{moisture:.1f}%\n"

            # Nutrients sensor (from casaverde_sim)
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
                elif "SET FAN1 OFF" in received:  # Fixed typo from original
                    fan_state = "OFF"
                response = f"FAN:{fan_state}\n"

            else:
                response = "ACK\n"

            sim_serial.write(response.encode("utf-8"))
            logging.info(f"Sent response: {response.strip()}")
            print(f"Sent response: {response.strip()}")

        # --- Simulation clock advance ---
        sim_time += SIM_STEP

        # --- Update simulated sensors ---
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
        temp1, temp2 = read_arduino_temps()
        temp1_str = f"{temp1:.1f}" if temp1 is not None else f"{probe_temp_1:.1f}"
        temp2_str = f"{temp2:.1f}" if temp2 is not None else f"{probe_temp_2:.1f}"
        status = (
            f"[{sim_time.strftime('%H:%M:%S')}] Probe1:{temp1_str}°C Probe2:{temp2_str}°C "
            f"Humidity:{humidity:.1f}% Water:{'OPEN' if water_valve_open else 'CLOSED'} "
            f"Solar:{solar_state} Moisture:{moisture:.1f}% Nutrients:{nutrients:.3f} "
            f"Relays:{relay_state}"
        )
        print(status)
        logging.info(status)

        time.sleep(0.1)

    except Exception as e:
        logging.error(f"Simulator error: {e}")
        print(f"Simulator error: {e}")
        break
