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

select_simulation() {
    sim_files=("$PROJECT_ROOT/casaverde_sim"/simulation_*.py)
    if [[ ${#sim_files[@]} -eq 0 ]]; then
        echo "No simulation Python files found in $PROJECT_ROOT/casaverde_sim"
        exit 1
    fi

    echo "Available simulations:"
    for i in "${!sim_files[@]}"; do
        printf "  %d) %s\n" $((i+1)) "$(basename "${sim_files[$i]}")"
    done

    read -rp "Select simulation number [1]: " choice
    if ! [[ "$choice" =~ ^[0-9]+$ ]] || ((choice < 1)) || ((choice > ${#sim_files[@]})); then
        choice=1
    fi
    SIM_PY="${sim_files[$((choice-1))]}"
    echo "Selected simulation: $(basename "$SIM_PY")"
}

scan_logs() {
    for log_file in "$@"; do
        if [[ -f "$log_file" ]]; then
            echo -e "\n--- Log scan: $log_file ---"
            grep -Ei "error|warning" "$log_file" || echo "No errors/warnings found."
        fi
    done
}

# ---------------------------
# INITIAL SETUP
# ---------------------------
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_OUTPUT="$PROJECT_ROOT/build_output/linux"
CONTROLLER_DIR="$BUILD_OUTPUT/casaverde_controller"
SERVER_DIR="$BUILD_OUTPUT/casaverde_server"

# ---------------------------
# CLEANUP SERIAL PORTS
# ---------------------------
PORT=/dev/ttyACM0
if lsof "$PORT" >/dev/null 2>&1; then
    echo "$PORT is busy, killing processes..."
    lsof "$PORT" | awk 'NR>1 {print $2}' | xargs -r sudo kill -9
    echo "Processes using $PORT have been killed."
fi

# ---------------------------
# SOCAT - virtual serial ports
# ---------------------------
echo "Creating virtual serial ports..."
SOCAT_LOG=$(mktemp)
socat -d -d PTY,link=/tmp/virtualcom0,raw,echo=0 PTY,link=/tmp/virtualcom1,raw,echo=0 &> "$SOCAT_LOG" &
SOCAT_PID=$!
sleep 1
echo "Virtual ports created: /tmp/virtualcom0 <-> /tmp/virtualcom1 (PID $SOCAT_PID)"

# ---------------------------
# PYTHON SIMULATION
# ---------------------------
select_simulation
echo "Starting Python simulation..."
if [[ -f "$PROJECT_ROOT/casaverde_sim/venv/bin/activate" ]]; then
    echo "Activating Python virtual environment..."
    # shellcheck source=/dev/null
    source "$PROJECT_ROOT/casaverde_sim/venv/bin/activate"
fi

SIM_LOG=$(mktemp)
python "$SIM_PY" &> "$SIM_LOG" &
SIM_PID=$!
echo "Python simulation running (PID $SIM_PID)"

# ---------------------------
# CASAVERDE SERVER
# ---------------------------
if [[ -f "$SERVER_DIR/casaverde_server" ]]; then
    echo "Starting Casaverde server..."
    SERVER_LOG=$(mktemp)
    (cd "$SERVER_DIR" && ./casaverde_server &> "$SERVER_LOG" &) 
    SERVER_PID=$!
    echo "Server running (PID $SERVER_PID)"
else
    echo "Warning: Server binary not found at $SERVER_DIR"
fi

# ---------------------------
# CASAVERDE CONTROLLER
# ---------------------------
if [[ -f "$CONTROLLER_DIR/casaverde_controller" ]]; then
    echo "Starting Casaverde controller..."
    CONTROLLER_LOG=$(mktemp)
    (cd "$CONTROLLER_DIR" && ./casaverde_controller &> "$CONTROLLER_LOG" &) 
    CONTROLLER_PID=$!
    echo "Controller running (PID $CONTROLLER_PID)"
else
    echo "Warning: Controller binary not found at $CONTROLLER_DIR"
fi

# ---------------------------
# NOTIFICATIONS & LOG SCAN
# ---------------------------
echo -e "\nAll components launched. Scanning logs for errors/warnings..."
scan_logs "$SOCAT_LOG" "$SIM_LOG" "${SERVER_LOG:-}" "${CONTROLLER_LOG:-}"

echo -e "\nSummary:"
echo "  SOCAT (virtual serial ports) PID: $SOCAT_PID"
echo "  Python simulation PID: $SIM_PID"
[[ -n "${SERVER_PID:-}" ]] && echo "  Server PID: $SERVER_PID"
[[ -n "${CONTROLLER_PID:-}" ]] && echo "  Controller PID: $CONTROLLER_PID"
echo -e "\nSimulation is running. Start Casaverde App manually in another terminal."
echo "Check individual logs if needed:"
echo "  SOCAT: $SOCAT_LOG"
echo "  Python simulation: $SIM_LOG"
[[ -n "${SERVER_LOG:-}" ]] && echo "  Server: $SERVER_LOG"
[[ -n "${CONTROLLER_LOG:-}" ]] && echo "  Controller: $CONTROLLER_LOG"

# ---------------------------
# KEEP SCRIPT RUNNING
# ---------------------------
wait "$SOCAT_PID" "$SIM_PID" "${SERVER_PID:-}" "${CONTROLLER_PID:-}"

