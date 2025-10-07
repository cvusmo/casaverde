#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Casaverde Build Script
# ============================================================

PROJECT_ROOT="/home/echo/projects/remote/casaverde"
BUILD_OUTPUT="${PROJECT_ROOT}/build_output"
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

mkdir -p "${BUILD_OUTPUT}/linux" "$LOG_DIR"
touch "$BUILD_LOG"

log_with_timestamp() {
  echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$BUILD_LOG"
}

kill_running_processes() {
  log_with_timestamp "Checking for running casaverde processes..."
  local pids
  pids=$(pgrep -f casaverde 2>/dev/null || true)
  if [[ -n "$pids" ]]; then
    log_with_timestamp "Terminating running casaverde processes (PIDs: $pids)..."
    for pid in $pids; do
      kill -9 "$pid" 2>/dev/null || true
    done
    log_with_timestamp "All casaverde processes terminated."
  else
    log_with_timestamp "No running casaverde processes found."
  fi
}

copy_certificates() {
  local cert_source="${PROJECT_ROOT}/cert.pem"
  local key_source="${PROJECT_ROOT}/key.pem"

  # Check if source certificate and key exist
  [[ -f "$cert_source" ]] || { log_with_timestamp "Certificate not found: $cert_source"; exit 1; }
  [[ -f "$key_source" ]] || { log_with_timestamp "Key not found: $key_source"; exit 1; }

  # Copy certificates and keys for each project
  for project in casaverde_server casaverde_app casaverde_controller; do
    local project_dir="${BUILD_OUTPUT}/linux/$project"
    mkdir -p "$project_dir" || { log_with_timestamp "Failed to create directory: $project_dir"; exit 1; }

    # Copy certificate for all projects
    cp "$cert_source" "$project_dir/server.crt" 2>&1 | tee -a "$BUILD_LOG" || {
      log_with_timestamp "Failed copying cert to $project_dir/server.crt"; exit 1;
    }
    chmod 600 "$project_dir/server.crt" 2>/dev/null || true

    # Copy key only for casaverde_server
    if [[ "$project" == "casaverde_server" ]]; then
      cp "$key_source" "$project_dir/server.key" 2>&1 | tee -a "$BUILD_LOG" || {
        log_with_timestamp "Failed copying key to $project_dir/server.key"; exit 1;
      }
      chmod 600 "$project_dir/server.key" 2>/dev/null || true
    fi
  done

  # Copy for release mode directories
  if [[ "$action" == "release" || "$action" == "deploy" ]]; then
    for project in casaverde_server casaverde_app casaverde_controller; do
      local release_dir="$HOME/.config/casaverde/$project"
      mkdir -p "$release_dir" || { log_with_timestamp "Failed to create release directory: $release_dir"; exit 1; }
      cp "$cert_source" "$release_dir/server.crt" 2>&1 | tee -a "$BUILD_LOG" || {
        log_with_timestamp "Failed copying cert to $release_dir/server.crt"; exit 1;
      }
      chmod 600 "$release_dir/server.crt" 2>/dev/null || true
      if [[ "$project" == "casaverde_server" ]]; then
        cp "$key_source" "$release_dir/server.key" 2>&1 | tee -a "$BUILD_LOG" || {
          log_with_timestamp "Failed copying key to $release_dir/server.key"; exit 1;
        }
        chmod 600 "$release_dir/server.key" 2>/dev/null || true
      fi
    done
  fi
}

copy_configs() {
  for project in casaverde_server casaverde_app casaverde_controller; do
    local config_source="${PROJECT_ROOT}/${project}/config.toml"
    local config_dest="${BUILD_OUTPUT}/linux/$project/config.toml"
    mkdir -p "$(dirname "$config_dest")"
    if [[ -f "$config_source" ]]; then
      cp "$config_source" "$config_dest" 2>&1 | tee -a "$BUILD_LOG" || {
        log_with_timestamp "Failed copying config to $config_dest"; exit 1;
      }
      chmod 644 "$config_dest" 2>/dev/null || true
      log_with_timestamp "Copied config.toml for $project from $config_source to $config_dest"
    else
      log_with_timestamp "Config file not found: $config_source"
      exit 1
    fi
    if [[ "$action" == "release" || "$action" == "deploy" ]]; then
      local release_dir="$HOME/.config/casaverde/$project"
      mkdir -p "$release_dir"
      cp "$config_source" "$release_dir/config.toml" 2>&1 | tee -a "$BUILD_LOG" || {
        log_with_timestamp "Failed copying config to $release_dir/config.toml"; exit 1;
      }
      chmod 644 "$release_dir/config.toml" 2>/dev/null || true
    fi
  done
}

prompt_config() {
  echo "Use default settings? (IP: $DEFAULT_IP, Hostname: $DEFAULT_HOSTNAME, Port: $DEFAULT_PORT, Serial: $DEFAULT_SERIAL_PORT_REAL/$DEFAULT_SERIAL_PORT_SIM) (y/n)"
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
    read -rp "Enter Real serial port (default $DEFAULT_SERIAL_PORT_REAL): " SERIAL_PORT_REAL
    SERIAL_PORT_REAL=${SERIAL_PORT_REAL:-$DEFAULT_SERIAL_PORT_REAL}
    read -rp "Enter Sim serial port (default $DEFAULT_SERIAL_PORT_SIM): " SERIAL_PORT_SIM
    SERIAL_PORT_SIM=${SERIAL_PORT_SIM:-$DEFAULT_SERIAL_PORT_SIM}
  fi

  [[ ! "$SERVER_PORT" =~ ^[0-9]+$ ]] && { log_with_timestamp "Server port must be a number"; exit 1; }
  [[ ! "$APP_PORT" =~ ^[0-9]+$ ]] && { log_with_timestamp "App port must be a number"; exit 1; }
  [[ ! "$CONTROLLER_PORT" =~ ^[0-9]+$ ]] && { log_with_timestamp "Controller port must be a number"; exit 1; }
  [[ -z "$SERIAL_PORT_REAL" || -z "$SERIAL_PORT_SIM" ]] && { log_with_timestamp "Serial ports required"; exit 1; }

  copy_certificates
  copy_configs
}

verify_paths() {
  for project in casaverde_server casaverde_app casaverde_controller; do
    local config_path="${BUILD_OUTPUT}/linux/$project/config.toml"
    local cert_path="${BUILD_OUTPUT}/linux/$project/server.crt"
    local key_path="${BUILD_OUTPUT}/linux/$project/server.key"

    log_with_timestamp "Verifying paths for $project..."
    [[ -f "$config_path" ]] || { log_with_timestamp "Config not found: $config_path"; exit 1; }
    # Only check for server.crt and server.key for casaverde_server
    if [[ "$project" == "casaverde_server" ]]; then
      [[ -f "$cert_path" ]] || { log_with_timestamp "Certificate not found: $cert_path"; exit 1; }
      [[ -f "$key_path" ]] || { log_with_timestamp "Key not found: $key_path"; exit 1; }
      [[ -r "$key_path" ]] || { log_with_timestamp "Key not readable: $key_path"; exit 1; }
      [[ -r "$cert_path" ]] || { log_with_timestamp "Certificate not readable: $cert_path"; exit 1; }
    fi
    [[ -r "$config_path" ]] || {
      log_with_timestamp "Config not readable: config=$config_path"
      exit 1
    }
    log_with_timestamp "Paths verified for $project: config=$config_path"
  done
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
  pushd "$PROJECT_ROOT/$project" >/dev/null || {
    log_with_timestamp "Directory not found: ${PROJECT_ROOT}/${project}"
    exit 1
  }

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
    cross build $cargo_args 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Build failed for $project"; exit 1; }
  else
    cargo build $cargo_args 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Build failed for $project"; exit 1; }
  fi

  popd >/dev/null

  local bin_name="casaverde_${project##casaverde_}"
  local bin_path="$PROJECT_ROOT/target/$target/$mode/$bin_name"
  [[ -f "$bin_path" ]] || { log_with_timestamp "Binary not found: $bin_path"; exit 1; }
  mkdir -p "$output_dir/$project"
  cp "$bin_path" "$output_dir/$project/$bin_name" || { log_with_timestamp "Failed to copy binary for $project"; exit 1; }
  chmod +x "$output_dir/$project/$bin_name"
  log_with_timestamp "Built $project successfully"
}

deploy_project() {
  local project="$1"
  local output_dir="$2"
  local mode="${3:-debug}"

  local bin_name="casaverde_${project##casaverde_}"
  local bin_path="$PROJECT_ROOT/target/${TARGETS[linux]}/$mode/$bin_name"
  [[ -f "$bin_path" ]] || { log_with_timestamp "Binary not found for deploy: $bin_path"; exit 1; }

  mkdir -p "$output_dir/$project"
  cp "$bin_path" "$output_dir/$project/$bin_name" || { log_with_timestamp "Failed to copy binary for $project"; exit 1; }
  chmod +x "$output_dir/$project/$bin_name"
  log_with_timestamp "Deployed $project successfully"
}

main() {
  local action="${1:-}"
  [[ -z "$action" ]] && { log_with_timestamp "Usage: $0 [debug|release|deploy|clean]"; exit 1; }

  [[ "$action" =~ ^(debug|release|deploy)$ ]] && prompt_config

  kill_running_processes

  case "$action" in
    "debug")
      cargo build --verbose --manifest-path "$PROJECT_ROOT/Cargo.toml" --target "${TARGETS[linux]}" --all 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Workspace build failed"; exit 1; }
      for project in casaverde_server casaverde_controller casaverde_app; do
        build_project "$project" "debug" "${TARGETS[linux]}" "${BUILD_OUTPUT}/linux" "false"
      done
      verify_paths
      log_with_timestamp "Debug build complete"
      ;;
    "release")
      cargo build --verbose --manifest-path "$PROJECT_ROOT/Cargo.toml" --target "${TARGETS[linux]}" --release --all 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Workspace build failed"; exit 1; }
      for project in casaverde_server casaverde_controller casaverde_app; do
        build_project "$project" "release" "${TARGETS[linux]}" "${BUILD_OUTPUT}/linux" "false"
      done
      verify_paths
      log_with_timestamp "Release build complete"
      ;;
    "deploy")
      MODE="debug"
      TARGET="${TARGETS[linux]}"
      cargo build --verbose --manifest-path "$PROJECT_ROOT/Cargo.toml" --target "$TARGET" --all 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Workspace build failed"; exit 1; }
      for project in casaverde_server casaverde_controller casaverde_app; do
        build_project "$project" "$MODE" "$TARGET" "${BUILD_OUTPUT}/linux" "false"
        deploy_project "$project" "${BUILD_OUTPUT}/linux" "$MODE"
      done
      verify_paths
      log_with_timestamp "Deploy complete"
      ;;
    "clean")
      cargo clean --manifest-path "$PROJECT_ROOT/Cargo.toml" 2>&1 | tee -a "$BUILD_LOG" || true
      rm -rf "$BUILD_OUTPUT" || true
      log_with_timestamp "Clean complete"
      ;;
  esac

  # Autostart prompt
  echo
  read -rp "Do you want to automatically start the Casaverde server and controller? (y/n): " start_choice
  echo

  SERVER_BIN="${BUILD_OUTPUT}/linux/casaverde_server/casaverde_server"
  CONTROLLER_BIN="${BUILD_OUTPUT}/linux/casaverde_controller/casaverde_controller"
  APP_BIN="${BUILD_OUTPUT}/linux/casaverde_app/casaverde_app"

  if [[ "${start_choice,,}" == "y" || "${start_choice,,}" == "yes" ]]; then
    log_with_timestamp "Starting Casaverde server and controller..."

    if [[ -x "$SERVER_BIN" ]]; then
      SERVER_RUN_LOG="${LOG_DIR}/casaverde_server.log"
      (cd "${PROJECT_ROOT}" && nohup "${SERVER_BIN}" &>> "${SERVER_RUN_LOG}" &)
      sleep 1
      SERVER_PID=$(pgrep -f "${SERVER_BIN}" | head -n 1)
      if [[ -n "$SERVER_PID" && $(ps -p "$SERVER_PID" > /dev/null && echo "running") == "running" ]]; then
        log_with_timestamp "Server started (PID $SERVER_PID) → $SERVER_RUN_LOG"
      else
        log_with_timestamp "⚠ Failed to start server: $SERVER_BIN"
      fi
    else
      log_with_timestamp "⚠ Server binary not found: $SERVER_BIN"
    fi

    if [[ -x "$CONTROLLER_BIN" ]]; then
      CONTROLLER_RUN_LOG="${LOG_DIR}/casaverde_controller.log"
      (cd "${PROJECT_ROOT}" && nohup "${CONTROLLER_BIN}" &>> "${CONTROLLER_RUN_LOG}" &)
      sleep 1
      CONTROLLER_PID=$(pgrep -f "${CONTROLLER_BIN}" | head -n 1)
      if [[ -n "$CONTROLLER_PID" && $(ps -p "$CONTROLLER_PID" > /dev/null && echo "running") == "running" ]]; then
        log_with_timestamp "Controller started (PID $CONTROLLER_PID) → $CONTROLLER_RUN_LOG"
      else
        log_with_timestamp "⚠ Failed to start controller: $CONTROLLER_BIN"
      fi
    else
      log_with_timestamp "⚠ Controller binary not found: $CONTROLLER_BIN"
    fi

    echo
    log_with_timestamp "✅ Server and controller running in background."
    echo "Reminder: To start the Casaverde App manually, run:"
    echo "  cd ${PROJECT_ROOT} && ${APP_BIN}"
    echo
  else
    echo
    log_with_timestamp "Skipping auto-start."
    echo "Manually start with:"
    echo "  cd ${PROJECT_ROOT} && ${SERVER_BIN}"
    echo "  cd ${PROJECT_ROOT} && ${CONTROLLER_BIN}"
    echo
    echo "Then run the Casaverde App with:"
    echo "  cd ${PROJECT_ROOT} && ${APP_BIN}"
    echo
  fi
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi
