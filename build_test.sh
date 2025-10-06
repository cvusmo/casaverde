#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_OUTPUT="${PROJECT_ROOT}/build_output"
CONFIG_DIR="${HOME}/.config/casaverde_server"
APP_CONFIG_DIR="${HOME}/.config/casaverde_app"
LOG_DIR="${BUILD_OUTPUT}/linux/logs"
BUILD_LOG="${LOG_DIR}/build.log"

# Default SETUP
DEFAULT_IP="127.0.0.1"
DEFAULT_HOSTNAME="localhost"
DEFAULT_PORT="3003"
DEFAULT_SERIAL_PORT_REAL="/dev/ttyACM0"
DEFAULT_SERIAL_PORT_SIM="/tmp/virtualcom0"

# CUSTOM SETUP (initialized with defaults)
SERVER_IP="$DEFAULT_IP"
APP_IP="$DEFAULT_IP"
CONTROLLER_IP="$DEFAULT_IP"
SERVER_HOSTNAME="$DEFAULT_HOSTNAME"
APP_HOSTNAME="$DEFAULT_HOSTNAME"
CONTROLLER_HOSTNAME="$DEFAULT_HOSTNAME"

SERVER_PORT="$DEFAULT_PORT"
APP_PORT="$DEFAULT_PORT"
CONTROLLER_PORT="$DEFAULT_PORT"

SERIAL_PORT_REAL="$DEFAULT_SERIAL_PORT_REAL"
SERIAL_PORT_SIM="$DEFAULT_SERIAL_PORT_SIM"

# Target platforms
declare -A TARGETS
TARGETS["linux"]="x86_64-unknown-linux-gnu"
# TODO: add windows, raspberry pi, etc.

mkdir -p "${BUILD_OUTPUT}/linux"
mkdir -p "$LOG_DIR"

log_with_timestamp() {
  echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$BUILD_LOG"
}

generate_certificates() {
  local cert_path="$CONFIG_DIR/server.crt"
  local key_path="$CONFIG_DIR/server.key"
  local cn="$SERVER_HOSTNAME"  # Use hostname for CN

  if [[ -f "$cert_path" && -f "$key_path" ]]; then
    local existing_cn
    existing_cn=$(openssl x509 -in "$cert_path" -noout -subject | grep -o 'CN\s*=\s*[^\s]*' | cut -d'=' -f2 | tr -d ' ')
    if [[ "$existing_cn" == "$cn" ]]; then
      log_with_timestamp "Existing certificate matches CN=$cn, reusing it."
      chmod +r "$cert_path" "$key_path" 2>/dev/null || true
      mkdir -p "$APP_CONFIG_DIR" && chmod u+rwx "$APP_CONFIG_DIR" 2>/dev/null || true
      cp "$cert_path" "$APP_CONFIG_DIR/server.crt" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Failed copying cert"; exit 1; }
      return
    else
      log_with_timestamp "Existing certificate CN=$existing_cn, needs CN=$cn. Regenerating..."
    fi
  fi

  log_with_timestamp "Generating self-signed TLS certificate for CN=$cn..."
  mkdir -p "$CONFIG_DIR" || { log_with_timestamp "Error creating $CONFIG_DIR"; exit 1; }
  chmod u+rwx "$CONFIG_DIR" 2>/dev/null || true
  openssl req -x509 -newkey rsa:4096 -keyout "$key_path" -out "$cert_path" \
    -sha256 -days 3650 -nodes -subj "/CN=$cn" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Certificate generation failed"; exit 1; }
  chmod +r "$cert_path" "$key_path" 2>/dev/null || true
  mkdir -p "$APP_CONFIG_DIR" && chmod u+rwx "$APP_CONFIG_DIR" 2>/dev/null || true
  cp "$cert_path" "$APP_CONFIG_DIR/server.crt" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Failed copying cert"; exit 1; }
}

prompt_config() {
  echo "Use default settings? IP: $DEFAULT_IP, Hostname: $DEFAULT_HOSTNAME, Port: $DEFAULT_PORT, Real Serial: $DEFAULT_SERIAL_PORT_REAL, Sim Serial: $DEFAULT_SERIAL_PORT_SIM (y/n)"
  read -r use_defaults
  if [[ ! "$use_defaults" =~ ^[Yy]$ ]]; then
    read -rp "Enter Server IP (default $DEFAULT_IP): " SERVER_IP
    SERVER_IP=${SERVER_IP:-$DEFAULT_IP}
    read -rp "Enter Server Hostname (default $DEFAULT_HOSTNAME): " SERVER_HOSTNAME
    SERVER_HOSTNAME=${SERVER_HOSTNAME:-$DEFAULT_HOSTNAME}
    read -rp "Enter App IP (default $DEFAULT_IP): " APP_IP
    APP_IP=${APP_IP:-$DEFAULT_IP}
    read -rp "Enter App Hostname (default $DEFAULT_HOSTNAME): " APP_HOSTNAME
    APP_HOSTNAME=${APP_HOSTNAME:-$DEFAULT_HOSTNAME}
    read -rp "Enter Controller IP (default $DEFAULT_IP): " CONTROLLER_IP
    CONTROLLER_IP=${CONTROLLER_IP:-$DEFAULT_IP}
    read -rp "Enter Controller Hostname (default $DEFAULT_HOSTNAME): " CONTROLLER_HOSTNAME
    CONTROLLER_HOSTNAME=${CONTROLLER_HOSTNAME:-$DEFAULT_HOSTNAME}

    read -rp "Enter Server Port (default $DEFAULT_PORT): " SERVER_PORT
    SERVER_PORT=${SERVER_PORT:-$DEFAULT_PORT}
    read -rp "Enter App Port (default $DEFAULT_PORT): " APP_PORT
    APP_PORT=${APP_PORT:-$DEFAULT_PORT}
    read -rp "Enter Controller Port (default $DEFAULT_PORT): " CONTROLLER_PORT
    CONTROLLER_PORT=${CONTROLLER_PORT:-$DEFAULT_PORT}

    read -rp "Real serial port (default $DEFAULT_SERIAL_PORT_REAL): " SERIAL_PORT_REAL
    SERIAL_PORT_REAL=${SERIAL_PORT_REAL:-$DEFAULT_SERIAL_PORT_REAL}
    read -rp "Sim serial port (default $DEFAULT_SERIAL_PORT_SIM): " SERIAL_PORT_SIM
    SERIAL_PORT_SIM=${SERIAL_PORT_SIM:-$DEFAULT_SERIAL_PORT_SIM}
  fi

  [[ ! "$SERVER_PORT" =~ ^[0-9]+$ ]] && { log_with_timestamp "Server port must be a number"; exit 1; }
  [[ ! "$APP_PORT" =~ ^[0-9]+$ ]] && { log_with_timestamp "App port must be a number"; exit 1; }
  [[ ! "$CONTROLLER_PORT" =~ ^[0-9]+$ ]] && { log_with_timestamp "Controller port must be a number"; exit 1; }
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
server = "${SERVER_IP}:${SERVER_PORT}"
hostname = "${SERVER_HOSTNAME}"
EOF
      cp "$config_path" "$CONFIG_DIR/config.toml" || true
      ;;
    "casaverde_app")
      cat >"$config_path" <<EOF
server = "https://${APP_HOSTNAME}:${APP_PORT}"
hostname = "${APP_HOSTNAME}"
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
server = "https://${CONTROLLER_HOSTNAME}:${CONTROLLER_PORT}"
hostname = "${CONTROLLER_HOSTNAME}"
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

  local project_hostname="$DEFAULT_HOSTNAME"
  case "$project" in
    "casaverde_server") project_hostname="$SERVER_HOSTNAME" ;;
    "casaverde_app") project_hostname="$APP_HOSTNAME" ;;
    "casaverde_controller") project_hostname="$CONTROLLER_HOSTNAME" ;;
  esac

  export HOSTNAME="$project_hostname"
  log_with_timestamp "HOSTNAME set to $HOSTNAME for $project build"

  log_with_timestamp "Checking for uncommitted changes in $project..."
  pushd "$PROJECT_ROOT/$project" >/dev/null

  if git status --porcelain | grep -q .; then
    log_with_timestamp "Uncommitted changes detected in $project:"
    git status --short | tee -a "$BUILD_LOG"
    echo "Would you like to run 'cargo fix'? (y/n)"
    read -r fix_changes
    if [[ "$fix_changes" =~ ^[Yy]$ ]]; then
      log_with_timestamp "Running cargo fix for $project..."
      local cargo_args="--target $target"
      [[ "$mode" == "release" ]] && cargo_args="$cargo_args --release"
      if [[ "$use_cross" == "true" ]]; then
        cross fix $cargo_args 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "cargo fix failed"; exit 1; }
      else
        cargo fix $cargo_args 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "cargo fix failed"; exit 1; }
      fi
      if git status --porcelain | grep -q .; then
        log_with_timestamp "Uncommitted changes remain after cargo fix. Commit/stash first."
        exit 1
      fi
    else
      log_with_timestamp "User declined cargo fix. Proceeding."
    fi
  fi

  log_with_timestamp "Building $project for $target in $mode mode..."
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

deploy_project() {
  local project="$1"
  local output_dir="$2"

  local bin_name="casaverde_${project##casaverde_}"
  local bin_path="$PROJECT_ROOT/target/$TARGET/$MODE/$bin_name"
  [[ ! -f "$bin_path" ]] && { log_with_timestamp "Binary not found for deploy: $bin_path"; exit 1; }

  mkdir -p "$output_dir/$project"
  cp "$bin_path" "$output_dir/$project/$bin_name"
  chmod +x "$output_dir/$project/$bin_name"

  local config_src="$CONFIG_DIR/config.toml"
  if [[ -f "$config_src" ]] && grep -q "https://" "$config_src"; then
    generate_config "$project" "$output_dir/$project/config.toml"
    cp "$CONFIG_DIR/server.crt" "$output_dir/$project/" || true
    [[ "$project" == "casaverde_server" ]] && cp "$CONFIG_DIR/server.key" "$output_dir/$project/" || true
  else
    log_with_timestamp "Skipping config copy for $project (no https:// found)"
  fi

  log_with_timestamp "Deployed $project successfully"
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
      MODE="release"
      TARGET="${TARGETS[linux]}"
      for project in "casaverde_server" "casaverde_controller"; do
        build_project "$project" "$MODE" "$TARGET" "${BUILD_OUTPUT}/linux" "false"
        deploy_project "$project" "${BUILD_OUTPUT}/linux"
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

