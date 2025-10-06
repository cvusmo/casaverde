#!/usr/bin/env python3
import serial
import subprocess
import random
import time
import json
import logging
import os
from datetime import datetime, timedelta
from pathlib import Path

# --- Logging setup ---
log_path = "/home/echo/projects/remote/casaverde/build_output/linux/logs/python_sim.log"
try:
    os.makedirs(os.path.dirname(log_path), exist_ok=True)
    logging.basicConfig(
        filename=log_path,
        level=logging.INFO,
        format="%(asctime)s [%(levelname)s] %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )
    logging.info("Logging initialized successfully")
except Exception as e:
    print(f"Failed to initialize logging: {e}")
    logging.error(f"Failed to initialize logging: {e}")
    raise

# --- Serial setup (virtual port for simulation only) ---
try:
    logging.info("Attempting to open virtual serial port /tmp/virtualcom1")
    sim_serial = serial.Serial("/tmp/virtualcom1", baudrate=9600, timeout=1)
    logging.info("Virtual serial port /tmp/virtualcom1 opened successfully")
except Exception as e:
    logging.error(f"Failed to open serial port /tmp/virtualcom1: {e}")
    print(f"Failed to open serial port /tmp/virtualcom1: {e}")
    raise

# --- Simulation state ---
relay_state = {"relay-1": "OFF", "relay-2": "OFF", "relay-3": "OFF", "relay-4": "OFF"}
fan_state = "OFF"
water_valve_open = False
solar_state = "ON"
humidity = 70.0
probe_temp = 15.0  # Simulated fallback for blackbeard-probe
probe_temp_2 = 15.0  # Simulated fallback for blackbeard-probe2
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


def get_real_cpu_temp():
    """Fetch CPU temperature using sensors command."""
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
    """Read moisture and nutrients from casaverde_sim output."""
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
                    nutrients = data[0]["nutrients"]
                    logging.info(
                        f"Read sim data: moisture={moisture:.1f}%, nutrients={nutrients:.3f}"
                    )
        else:
            logging.warning(f"SIM_DATA_FILE does not exist: {SIM_DATA_FILE}")
    except Exception as e:
        logging.error(f"Error reading sim data: {e}")


while True:
    try:
        # --- Read casaverde_sim data ---
        logging.info("Reading casaverde_sim data")
        read_sim_data()

        # --- Read commands from controller ---
        logging.info("Checking for controller commands")
        received = sim_serial.readline().decode("utf-8").strip()
        if received:
            logging.info(f"Received from controller: {received}")
            print(f"Received from controller: {received}")

            # CPU temperature (real data from sensors)
            if "blackbeard-cpu" in received.lower():
                cpu_temp = get_real_cpu_temp()
                response = (
                    f"TEMP:{cpu_temp:.2f}\n" if cpu_temp is not None else "TEMP:ERROR\n"
                )

            # Probe temperature (simulated for blackbeard-probe)
            elif "blackbeard-probe" in received.lower():
                response = f"TEMP_PROBE:{probe_temp:.1f}\n"
                logging.info(
                    f"Responding with simulated probe temp: {probe_temp:.1f}°C"
                )

            # Second probe (simulated for blackbeard-probe2)
            elif "blackbeard-probe2" in received.lower():
                response = f"TEMP_PROBE2:{probe_temp_2:.1f}\n"
                logging.info(
                    f"Responding with simulated probe2 temp: {probe_temp_2:.1f}°C"
                )

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
                elif "SET FAN1 OFF" in received:
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
        probe_temp += random.gauss(0, 0.2)
        probe_temp = max(5.0, min(25.0, probe_temp))
        probe_temp_2 += random.gauss(0, 0.2)
        probe_temp_2 = max(5.0, min(25.0, probe_temp_2))
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
        cpu_temp = get_real_cpu_temp()
        cpu_temp_str = f"{cpu_temp:.1f}" if cpu_temp is not None else "ERROR"
        status = (
            f"[{sim_time.strftime('%H:%M:%S')}] CPU:{cpu_temp_str}°C Probe1:{probe_temp:.1f}°C "
            f"Probe2:{probe_temp_2:.1f}°C Humidity:{humidity:.1f}% "
            f"Water:{'OPEN' if water_valve_open else 'CLOSED'} Solar:{solar_state} "
            f"Moisture:{moisture:.1f}% Nutrients:{nutrients:.3f} Relays:{relay_state}"
        )
        print(status)
        logging.info(status)

        time.sleep(0.1)  # Fast simulation loop

    except Exception as e:
        logging.error(f"Simulator error: {e}")
        print(f"Simulator error: {e}")
        raise
