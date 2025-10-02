import serial  # pip install pyserial
import subprocess
import random
import re

# Function to fetch real CPU temperature using `sensors`
def get_real_cpu_temp():
    try:
        result = subprocess.run(['sensors'], capture_output=True, text=True)
        output = result.stdout
        # Parse "Package id 0: +73.0°C" from your sensors output
        cpu_match = re.search(r'Package id 0:\s*\+\d+\.\d+°C', output)
        if cpu_match:
            temp_str = re.search(r'\+\d+\.\d+°C', cpu_match.group(0)).group(0)
            return float(temp_str.strip('+°C'))
        return None
    except Exception as e:
        print(f"Error fetching CPU temp: {e}")
        return None

# Configure serial to match your app (adjust baudrate if needed)
ser = serial.Serial('/tmp/virtualcom1', baudrate=9600, timeout=1)

# Track relay state for relay-1
relay_state = "OFF"

print("Simulator started. Waiting for commands...")

while True:
    try:
        received = ser.readline().decode('utf-8').strip()  # Read command from app
        if received:
            print(f"Received from app: {received}")

            # Simulate response based on device ID
            if "blackbeard-cpu" in received.lower():
                cpu_temp = get_real_cpu_temp()
                response = f"TEMP:{cpu_temp:.2f}\n" if cpu_temp is not None else "TEMP:ERROR\n"
            elif "blackbeard-gpu" in received.lower():
                simulated_gpu_temp = random.uniform(40.0, 60.0)  # Realistic GPU temp range
                response = f"TEMP:{simulated_gpu_temp:.2f}\n"
            elif "solar-1" in received.lower():
                simulated_solar_power = random.uniform(50.0, 150.0)  # Simulate 50-150W
                response = f"SOLAR:{simulated_solar_power:.1f}W\n"
            elif "relay-1" in received.lower():
                # Handle relay commands (e.g., "SET relay-1 ON" or queries)
                if "SET relay-1 ON" in received:
                    relay_state = "ON"  # No need for global, as relay_state is already global
                    response = "RELAY:OK\n"
                elif "SET relay-1 OFF" in received:
                    relay_state = "OFF"
                    response = "RELAY:OK\n"
                else:
                    response = f"RELAY:{relay_state}\n"  # Return current state
            else:
                response = "ACK\n"  # Default for unrecognized commands

            ser.write(response.encode('utf-8'))
            print(f"Sent response: {response.strip()}")
    except Exception as e:
        print(f"Error: {e}")
