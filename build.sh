#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_OUTPUT="${PROJECT_ROOT}/build_output"
CONFIG_DIR="${HOME}/.config/casaverde_server"
APP_CONFIG_DIR="${HOME}/.config/casaverde_app"
LOG_DIR="${BUILD_OUTPUT}/linux/logs"
BUILD_LOG="${LOG_DIR}/build.log"
DEFAULT_IP="10.0.0.6"
DEFAULT_PORT="3003"
DEFAULT_SERIAL_PORT_REAL="/dev/ttyACM0"
DEFAULT_SERIAL_PORT_SIM="/tmp/virtualcom0"

SERVER_URL="${DEFAULT_IP}:${DEFAULT_PORT}"
APP_URL="${DEFAULT_IP}:${DEFAULT_PORT}"
CONTROLLER_URL="${DEFAULT_IP}:${DEFAULT_PORT}"
PORT="${DEFAULT_PORT}"
SERIAL_PORT_REAL="${DEFAULT_SERIAL_PORT_REAL}"
SERIAL_PORT_SIM="${DEFAULT_SERIAL_PORT_SIM}"

# Define target platform for native Linux
declare -A TARGETS
TARGETS["linux"]="x86_64-unknown-linux-gnu"
#TODO: add windows and others
# TARGETS["windows"]="x86_64-pc-windows-gnu"
# TARGETS["raspberry_pi_arm"]="armv7-unknown-linux-gnueabihf"
# TARGETS["raspberry_pi_arm64"]="aarch64-unknown-linux-gnu"

mkdir -p "${BUILD_OUTPUT}/linux"
mkdir -p "$LOG_DIR"

log_with_timestamp() {
  echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$BUILD_LOG"
}

generate_certificates() {
  local cert_path="$CONFIG_DIR/server.crt"
  local key_path="$CONFIG_DIR/server.key"
  if [[ ! -f "$cert_path" || ! -f "$key_path" ]]; then
    log_with_timestamp "Generating self-signed TLS certificate..."
    mkdir -p "$CONFIG_DIR" || { log_with_timestamp "Error creating $CONFIG_DIR"; exit 1; }
    chmod u+rwx "$CONFIG_DIR" 2>/dev/null || true
    openssl req -x509 -newkey rsa:4096 -keyout "$key_path" -out "$cert_path" \
      -sha256 -days 3650 -nodes -subj "/CN=casaverde.local" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Certificate generation failed"; exit 1; }
    log_with_timestamp "Certificate generated at $cert_path"
  fi
  chmod +r "$cert_path" "$key_path" 2>/dev/null || true
  mkdir -p "$APP_CONFIG_DIR" && chmod u+rwx "$APP_CONFIG_DIR" 2>/dev/null || true
  cp "$cert_path" "$APP_CONFIG_DIR/server.crt" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Failed copying cert"; exit 1; }
}

prompt_config() {
  echo "Use default settings? IP: $DEFAULT_IP, Port: $DEFAULT_PORT, Real Serial: $DEFAULT_SERIAL_PORT_REAL, Sim Serial: $DEFAULT_SERIAL_PORT_SIM (y/n)"
  read -r use_defaults
  if [[ ! "$use_defaults" =~ ^[Yy]$ ]]; then
    read -rp "Enter IP (default $DEFAULT_IP): " ip
    ip=${ip:-$DEFAULT_IP}
    read -rp "Enter Port (default $DEFAULT_PORT): " PORT
    PORT=${PORT:-$DEFAULT_PORT}
    read -rp "Real serial port (default $DEFAULT_SERIAL_PORT_REAL): " SERIAL_PORT_REAL
    SERIAL_PORT_REAL=${SERIAL_PORT_REAL:-$DEFAULT_SERIAL_PORT_REAL}
    read -rp "Sim serial port (default $DEFAULT_SERIAL_PORT_SIM): " SERIAL_PORT_SIM
    SERIAL_PORT_SIM=${SERIAL_PORT_SIM:-$DEFAULT_SERIAL_PORT_SIM}
    SERVER_URL="$ip:$PORT"
    APP_URL="$ip:$PORT"
    CONTROLLER_URL="$ip:$PORT"
  fi
  [[ -z "$SERVER_URL" || -z "$APP_URL" || -z "$CONTROLLER_URL" ]] && { log_with_timestamp "All URLs must be set"; exit 1; }
  [[ ! "$PORT" =~ ^[0-9]+$ ]] && { log_with_timestamp "Port must be a number"; exit 1; }
  [[ -z "$SERIAL_PORT_REAL" || -z "$SERIAL_PORT_SIM" ]] && { log_with_timestamp "Serial ports required"; exit 1; }
  generate_certificates
}

generate_config() {
  local project="$1"
  local config_path="$2"
  mkdir -p "$(dirname "$config_path")"
  case "$project" in
    "casaverde_server")
      cat >"$config_path" <<EOF
server = "$SERVER_URL"
EOF
      cp "$config_path" "$CONFIG_DIR/config.toml" || true
      ;;
    "casaverde_app")
      cat >"$config_path" <<EOF
server = "$APP_URL"
[[configs]]
id = "blackbeard-cpu"
type = "temperature"
endpoint = "/sensor_data"
interval = 15
serial_port = "$SERIAL_PORT_REAL"

[[configs]]
id = "solar-1"
type = "solar"
endpoint = "/sensor_data"
serial_port = "$SERIAL_PORT_SIM"
interval = 15

[[configs]]
id = "moisture-1"
type = "moisture"
endpoint = "/sensor_data"
serial_port = "$SERIAL_PORT_SIM"
interval = 15

[[configs]]
id = "humidity-1"
type = "humidity"
endpoint = "/sensor_data"
serial_port = "$SERIAL_PORT_SIM"
interval = 15

[[configs]]
id = "water-1"
type = "water"
endpoint = "/sensor_data"
serial_port = "$SERIAL_PORT_SIM"
interval = 15

[[configs]]
id = "relay-1"
type = "relay"
endpoint = "/relay_control"
serial_port = "$SERIAL_PORT_SIM"
interval = 15

[[configs]]
id = "blackbeard-probe"
type = "temperature"
endpoint = "/sensor_data"
serial_port = "$SERIAL_PORT_REAL"
interval = 15
EOF
      mkdir -p "$APP_CONFIG_DIR"
      cp "$config_path" "$APP_CONFIG_DIR/config.toml" || true
      ;;
    "casaverde_controller")
      cat >"$config_path" <<EOF
server = "$CONTROLLER_URL"
controller_id = "blackbeard-pi"
serial_port = "$SERIAL_PORT_REAL"
light_relay_id = "relay-1"
light_on_hours = 16
light_off_hours = 8
EOF
      ;;
  esac
}

build_project() {
  local project="$1"
  local mode="$2"
  local target="$3"
  local output_dir="$4"
  local use_cross="$5"

  log_with_timestamp "Building $project for $target in $mode mode..."
  pushd "$PROJECT_ROOT/$project" >/dev/null
  local cargo_args="--target $target"
  [[ "$mode" == "release" ]] && cargo_args="$cargo_args --release"

  if [[ "$use_cross" == "true" ]]; then
    cross build $cargo_args 2>&1 | tee -a "$BUILD_LOG" || exit 1
  else
    cargo build $cargo_args 2>&1 | tee -a "$BUILD_LOG" || exit 1
  fi
  popd >/dev/null

  local bin_name="casaverde_${project##casaverde_}"
  local bin_path="$PROJECT_ROOT/target/$target/$mode/$bin_name"
  [[ ! -f "$bin_path" ]] && { log_with_timestamp "Binary not found: $bin_path"; exit 1; }
  mkdir -p "$output_dir/$project"
  cp "$bin_path" "$output_dir/$project/$bin_name"
  chmod +x "$output_dir/$project/$bin_name"
  generate_config "$project" "$output_dir/$project/config.toml"
  cp "$CONFIG_DIR/server.crt" "$output_dir/$project/" || true
  [[ "$project" == "casaverde_server" ]] && cp "$CONFIG_DIR/server.key" "$output_dir/$project/" || true
  log_with_timestamp "Built $project successfully"
}

main() {
  local action="${1:-}"
  [[ -z "$action" ]] && { log_with_timestamp "Usage: $0 [debug|release|deploy|clean]"; exit 1; }

  [[ "$action" =~ ^(debug|release|deploy)$ ]] && prompt_config

  case "$action" in
    "debug"|"release")
      for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
        build_project "$project" "$action" "${TARGETS[linux]}" "${BUILD_OUTPUT}/linux" "false"
      done
      log_with_timestamp "$action build complete"
      ;;
    "deploy")
      for project in "casaverde_server" "casaverde_controller"; do
        build_project "$project" "release" "${TARGETS[linux]}" "${BUILD_OUTPUT}/linux" "false"
      done
      ;;
    "clean")
      cargo clean --manifest-path "$PROJECT_ROOT/Cargo.toml" 2>&1 | tee -a "$BUILD_LOG" || true
      rm -rf "$BUILD_OUTPUT" || true
      log_with_timestamp "Clean complete"
      ;;
  esac
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi

