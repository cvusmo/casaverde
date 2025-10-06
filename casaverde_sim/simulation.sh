#!/usr/bin/env bash
set -euo pipefail

# ---------------------------
# FUNCTIONS
# ---------------------------
cleanup() {
    echo -e "\n[$(date '+%Y-%m-%d %H:%M:%S')] Stopping simulation..."
    if [[ -n "${SOCAT_PID:-}" ]]; then
        kill "$SOCAT_PID" 2>/dev/null && log_with_timestamp "Stopped socat (PID $SOCAT_PID)" || true
    fi
    if [[ -n "${SIM_PID:-}" ]]; then
        kill "$SIM_PID" 2>/dev/null && log_with_timestamp "Stopped Python simulation (PID $SIM_PID)" || true
    fi
    if [[ -n "${RUST_SIM_PID:-}" ]]; then
        kill "$RUST_SIM_PID" 2>/dev/null && log_with_timestamp "Stopped casaverde_sim (PID $RUST_SIM_PID)" || true
    fi
    if [[ -n "${SERVER_PID:-}" ]]; then
        kill "$SERVER_PID" 2>/dev/null && log_with_timestamp "Stopped casaverde_server (PID $SERVER_PID)" || true
    fi
    if [[ -n "${CONTROLLER_PID:-}" ]]; then
        kill "$CONTROLLER_PID" 2>/dev/null && log_with_timestamp "Stopped casaverde_controller (PID $CONTROLLER_PID)" || true
    fi
    exit 0
}
trap cleanup SIGINT SIGTERM

check_python_deps() {
    for dep in serial; do
        python3 -c "import $dep" 2>/dev/null || {
            log_with_timestamp "Python module '$dep' not found in venv. Install with: pip install pyserial"
            exit 1
        }
    done
}

check_arduino() {
    if [[ -c "/dev/ttyACM0" ]]; then
        log_with_timestamp "Arduino detected on /dev/ttyACM0"
        return 0
    else
        log_with_timestamp "Error: Arduino not detected on /dev/ttyACM0"
        exit 1
    fi
}

generate_sim_config() {
    local config_path="$BUILD_OUTPUT/casaverde_sim/config.toml"
    mkdir -p "$(dirname "$config_path")"
    cat >"$config_path" <<EOF
[simulation]
width = 10
height = 10
moisture_decay = 0.01
nutrient_decay = 0.005
growth_rate = 0.002
[logging]
level = "info"
EOF
    log_with_timestamp "Generated casaverde_sim config: $config_path"
}

scan_logs() {
    for log_file in "$@"; do
        if [[ -f "$log_file" ]]; then
            echo -e "\n--- Log scan: $log_file ---"
            grep -Ei "error|warning" "$log_file" || echo "No errors/warnings found."
        fi
    done
}

log_with_timestamp() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$SIM_LOG_DIR/simulation.log"
}

# ---------------------------
# INITIAL SETUP
# ---------------------------
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_OUTPUT="$PROJECT_ROOT/build_output/linux"
CONTROLLER_DIR="$BUILD_OUTPUT/casaverde_controller"
SERVER_DIR="$BUILD_OUTPUT/casaverde_server"
SIM_LOG_DIR="$PROJECT_ROOT/build_output/linux/logs"
SIM_PY="$PROJECT_ROOT/casaverde_sim/py/simulation.py"
SIM_BINARY="$BUILD_OUTPUT/casaverde_sim/casaverde_sim"

mkdir -p "$SIM_LOG_DIR"

# ---------------------------
# CHECK ARDUINO
# ---------------------------
check_arduino

# ---------------------------
# SOCAT - virtual serial ports
# ---------------------------
log_with_timestamp "Creating virtual serial ports..."
SOCAT_LOG="$SIM_LOG_DIR/socat.log"
socat -d -d PTY,link=/tmp/virtualcom0,raw,echo=0 PTY,link=/tmp/virtualcom1,raw,echo=0 &> "$SOCAT_LOG" &
SOCAT_PID=$!
sleep 1
if ! ps -p "$SOCAT_PID" > /dev/null; then
    log_with_timestamp "Error: Failed to start socat (PID $SOCAT_PID)"
    exit 1
fi
log_with_timestamp "Virtual ports created: /tmp/virtualcom0 <-> /tmp/virtualcom1 (PID $SOCAT_PID)"

# ---------------------------
# RUST SIMULATION (casaverde_sim)
# ---------------------------
if [[ -f "$SIM_BINARY" ]]; then
    log_with_timestamp "Starting Rust casaverde_sim binary..."
    generate_sim_config
    SIM_LOG="$SIM_LOG_DIR/casaverde_sim.log"
    (cd "$BUILD_OUTPUT/casaverde_sim" && ./casaverde_sim &> "$SIM_LOG" &) 
    RUST_SIM_PID=$!
    sleep 1
    if ! ps -p "$RUST_SIM_PID" > /dev/null; then
        log_with_timestamp "Error: Failed to start casaverde_sim (PID $RUST_SIM_PID)"
        exit 1
    fi
    log_with_timestamp "Rust simulation running (PID $RUST_SIM_PID)"
else
    log_with_timestamp "Error: Rust simulation binary not found at $SIM_BINARY"
    exit 1
fi

# ---------------------------
# PYTHON SIMULATION (simulation_5.py)
# ---------------------------
log_with_timestamp "Starting Python simulation: $(basename "$SIM_PY")..."
if [[ -f "$PROJECT_ROOT/casaverde_sim/venv/bin/activate" ]]; then
    log_with_timestamp "Activating Python virtual environment..."
    source "$PROJECT_ROOT/casaverde_sim/venv/bin/activate"
    check_python_deps
else
    log_with_timestamp "Error: Python virtual environment not found at $PROJECT_ROOT/casaverde_sim/venv"
    exit 1
fi
SIM_LOG="$SIM_LOG_DIR/python_sim.log"
python3 "$SIM_PY" &> "$SIM_LOG" &
SIM_PID=$!
sleep 1
if ! ps -p "$SIM_PID" > /dev/null; then
    log_with_timestamp "Error: Failed to start Python simulation (PID $SIM_PID)"
    exit 1
fi
log_with_timestamp "Python simulation running (PID $SIM_PID)"

# ---------------------------
# CASAVERDE SERVER
# ---------------------------
if [[ -f "$SERVER_DIR/casaverde_server" ]]; then
    log_with_timestamp "Starting Casaverde server..."
    SERVER_LOG="$SIM_LOG_DIR/casaverde_server.log"
    (cd "$SERVER_DIR" && ./casaverde_server &> "$SERVER_LOG" &) 
    SERVER_PID=$!
    sleep 1
    if ! ps -p "$SERVER_PID" > /dev/null; then
        log_with_timestamp "Error: Failed to start casaverde_server (PID $SERVER_PID)"
        exit 1
    fi
    log_with_timestamp "Server running (PID $SERVER_PID)"
else
    log_with_timestamp "Error: Server binary not found at $SERVER_DIR"
    exit 1
fi

# ---------------------------
# CASAVERDE CONTROLLER
# ---------------------------
if [[ -f "$CONTROLLER_DIR/casaverde_controller" ]]; then
    log_with_timestamp "Starting Casaverde controller..."
    CONTROLLER_LOG="$SIM_LOG_DIR/casaverde_controller.log"
    (cd "$CONTROLLER_DIR" && SIMULATION_MODE=1 ./casaverde_controller &> "$CONTROLLER_LOG" &) 
    CONTROLLER_PID=$!
    sleep 1
    if ! ps -p "$CONTROLLER_PID" > /dev/null; then
        log_with_timestamp "Error: Failed to start casaverde_controller (PID $CONTROLLER_PID)"
        exit 1
    fi
    log_with_timestamp "Controller running (PID $CONTROLLER_PID)"
else
    log_with_timestamp "Error: Controller binary not found at $CONTROLLER_DIR"
    exit 1
fi

# ---------------------------
# NOTIFICATIONS & LOG SCAN
# ---------------------------
log_with_timestamp "All components launched. Scanning logs for errors/warnings..."
scan_logs "$SOCAT_LOG" "$SIM_LOG" "$SIM_LOG_DIR/casaverde_sim.log" "${SERVER_LOG:-}" "${CONTROLLER_LOG:-}"

echo -e "\nSummary:"
echo "  SOCAT (virtual serial ports) PID: ${SOCAT_PID:-none}"
echo "  Rust Simulation PID: ${RUST_SIM_PID:-none}"
echo "  Python Simulation PID: ${SIM_PID:-none}"
echo "  Server PID: ${SERVER_PID:-none}"
echo "  Controller PID: ${CONTROLLER_PID:-none}"
echo -e "\nSimulation is running. Start Casaverde App manually in another terminal."
echo "Check individual logs if needed:"
echo "  SOCAT: ${SOCAT_LOG:-none}"
echo "  Rust Simulation: $SIM_LOG_DIR/casaverde_sim.log"
echo "  Python Simulation: ${SIM_LOG:-none}"
echo "  Server: ${SERVER_LOG:-none}"
echo "  Controller: ${CONTROLLER_LOG:-none}"

# ---------------------------
# KEEP SCRIPT RUNNING
# ---------------------------
wait "${SOCAT_PID:-}" "${RUST_SIM_PID:-}" "${SIM_PID:-}" "${SERVER_PID:-}" "${CONTROLLER_PID:-}"
