#!/usr/bin/env bash
set -euo pipefail

# ---------------------------
# FUNCTIONS
# ---------------------------
cleanup() {
    echo -e "\nStopping simulation..."
    [[ -n "${SOCAT_PID:-}" ]] && kill "$SOCAT_PID" 2>/dev/null || true
    [[ -n "${SIM_PID:-}" ]] && kill "$SIM_PID" 2>/dev/null || true
    [[ -n "${SERVER_PID:-}" ]] && kill "$SERVER_PID" 2>/dev/null || true
    [[ -n "${CONTROLLER_PID:-}" ]] && kill "$CONTROLLER_PID" 2>/dev/null || true
    exit 0
}
trap cleanup SIGINT SIGTERM

check_python_deps() {
    for dep in serial; do
        python3 -c "import $dep" 2>/dev/null || {
            echo "Python module '$dep' not found in venv. Install with: pip install $dep"
            exit 1
        }
    done
}

select_simulation() {
    sim_files=("$PROJECT_ROOT/casaverde_sim/py"/simulation_*.py)
    if [[ ${#sim_files[@]} -eq 0 ]]; then
        echo "No simulation Python files found in $PROJECT_ROOT/casaverde_sim/py"
        exit 1
    fi

    echo "Available Python simulations:"
    for i in "${!sim_files[@]}"; do
        printf "  %d) %s\n" $((i+1)) "$(basename "${sim_files[$i]}")"
    done

    read -rp "Select simulation number [1] or 'rust' for casaverde_sim binary: " choice
    if [[ "$choice" == "rust" ]] && [[ -f "$BUILD_OUTPUT/casaverde_sim/casaverde_sim" ]]; then
        SIM_BINARY="$BUILD_OUTPUT/casaverde_sim/casaverde_sim"
        echo "Selected Rust simulation: casaverde_sim"
    elif ! [[ "$choice" =~ ^[0-9]+$ ]] || ((choice < 1)) || ((choice > ${#sim_files[@]})); then
        SIM_PY="${sim_files[0]}"
        echo "Selected default Python simulation: $(basename "$SIM_PY")"
    else
        SIM_PY="${sim_files[$((choice-1))]}"
        echo "Selected Python simulation: $(basename "$SIM_PY")"
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
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_OUTPUT="$PROJECT_ROOT/build_output/linux"
CONTROLLER_DIR="$BUILD_OUTPUT/casaverde_controller"
SERVER_DIR="$BUILD_OUTPUT/casaverde_server"
SIM_LOG_DIR="$HOME/casaverde/target/build/log"
SIM_BINARY=""
SIM_PY=""

mkdir -p "$SIM_LOG_DIR"

# ---------------------------
# SOCAT - virtual serial ports
# ---------------------------
log_with_timestamp "Creating virtual serial ports..."
SOCAT_LOG="$SIM_LOG_DIR/socat.log"
socat -d -d PTY,link=/tmp/virtualcom0,raw,echo=0 PTY,link=/tmp/virtualcom1,raw,echo=0 &> "$SOCAT_LOG" &
SOCAT_PID=$!
sleep 1
log_with_timestamp "Virtual ports created: /tmp/virtualcom0 <-> /tmp/virtualcom1 (PID $SOCAT_PID)"

# ---------------------------
# SIMULATION
# ---------------------------
select_simulation
if [[ -n "$SIM_BINARY" ]]; then
    log_with_timestamp "Starting Rust casaverde_sim binary..."
    generate_sim_config
    SIM_LOG="$SIM_LOG_DIR/casaverde_sim.log"
    (cd "$BUILD_OUTPUT/casaverde_sim" && ./casaverde_sim &> "$SIM_LOG" &) 
    SIM_PID=$!
    log_with_timestamp "Rust simulation running (PID $SIM_PID)"
else
    log_with_timestamp "Starting Python simulation..."
    if [[ -f "$PROJECT_ROOT/casaverde_sim/venv/bin/activate" ]]; then
        log_with_timestamp "Activating Python virtual environment..."
        # shellcheck source=/dev/null
        source "$PROJECT_ROOT/casaverde_sim/venv/bin/activate"
        check_python_deps
    else
        log_with_timestamp "Error: Python virtual environment not found at $PROJECT_ROOT/casaverde_sim/venv"
        exit 1
    fi
    SIM_LOG="$SIM_LOG_DIR/python_sim.log"
    python3 "$SIM_PY" &> "$SIM_LOG" &
    SIM_PID=$!
    log_with_timestamp "Python simulation running (PID $SIM_PID)"
fi

# ---------------------------
# CASAVERDE SERVER
# ---------------------------
if [[ -f "$SERVER_DIR/casaverde_server" ]]; then
    log_with_timestamp "Starting Casaverde server..."
    SERVER_LOG="$SIM_LOG_DIR/casaverde_server.log"
    (cd "$SERVER_DIR" && ./casaverde_server &> "$SERVER_LOG" &) 
    SERVER_PID=$!
    log_with_timestamp "Server running (PID $SERVER_PID)"
else
    log_with_timestamp "Warning: Server binary not found at $SERVER_DIR"
fi

# ---------------------------
# CASAVERDE CONTROLLER
# ---------------------------
if [[ -f "$CONTROLLER_DIR/casaverde_controller" ]]; then
    log_with_timestamp "Starting Casaverde controller..."
    CONTROLLER_LOG="$SIM_LOG_DIR/casaverde_controller.log"
    (cd "$CONTROLLER_DIR" && SIMULATION_MODE=1 ./casaverde_controller &> "$CONTROLLER_LOG" &) 
    CONTROLLER_PID=$!
    log_with_timestamp "Controller running (PID $CONTROLLER_PID)"
else
    log_with_timestamp "Warning: Controller binary not found at $CONTROLLER_DIR"
fi

# ---------------------------
# NOTIFICATIONS & LOG SCAN
# ---------------------------
log_with_timestamp "All components launched. Scanning logs for errors/warnings..."
scan_logs "$SOCAT_LOG" "$SIM_LOG" "${SERVER_LOG:-}" "${CONTROLLER_LOG:-}"

echo -e "\nSummary:"
echo "  SOCAT (virtual serial ports) PID: $SOCAT_PID"
echo "  Simulation PID: $SIM_PID"
[[ -n "${SERVER_PID:-}" ]] && echo "  Server PID: $SERVER_PID"
[[ -n "${CONTROLLER_PID:-}" ]] && echo "  Controller PID: $CONTROLLER_PID"
echo -e "\nSimulation is running. Start Casaverde App manually in another terminal."
echo "Check individual logs if needed:"
echo "  SOCAT: $SOCAT_LOG"
echo "  Simulation: $SIM_LOG"
[[ -n "${SERVER_LOG:-}" ]] && echo "  Server: $SERVER_LOG"
[[ -n "${CONTROLLER_LOG:-}" ]] && echo "  Controller: $CONTROLLER_LOG"

# ---------------------------
# KEEP SCRIPT RUNNING
# ---------------------------
wait "$SOCAT_PID" "$SIM_PID" "${SERVER_PID:-}" "${CONTROLLER_PID:-}"
