#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_OUTPUT="${PROJECT_ROOT}/build_output"
TESTING_ROOT="${PROJECT_ROOT}/casaverde_test"
CONFIG_DIR="${HOME}/.config/casaverde_server"
BUILD_LOG="${PROJECT_ROOT}/build.log"
DEFAULT_IP="127.0.0.1"
DEFAULT_PORT="3003"

# Default values for URLs and ports
SERVER_URL="https://${DEFAULT_IP}:${DEFAULT_PORT}"
APP_URL="https://${DEFAULT_IP}:${DEFAULT_PORT}"
CONTROLLER_URL="https://${DEFAULT_IP}:${DEFAULT_PORT}"
PORT="${DEFAULT_PORT}"

# Define all target platforms and their Rust targets
declare -A TARGETS
TARGETS["linux"]="x86_64-unknown-linux-gnu"
TARGETS["windows"]="x86_64-pc-windows-gnu"
TARGETS["raspberry_pi_arm"]="armv7-unknown-linux-gnueabihf"
TARGETS["raspberry_pi_arm64"]="aarch64-unknown-linux-gnu"

# Output directories for each platform
mkdir -p "${BUILD_OUTPUT}/linux" "${BUILD_OUTPUT}/windows" "${BUILD_OUTPUT}/raspberry_pi/arm" "${BUILD_OUTPUT}/raspberry_pi/arm64"

log_with_timestamp() {
  echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$BUILD_LOG"
}

prompt_config() {
  echo "Do you want to use default settings? (IP: $DEFAULT_IP, Port: $DEFAULT_PORT) (y/n)"
  read -r use_defaults
  if [[ ! "$use_defaults" =~ ^[Yy]$ ]]; then
    echo "Enter the IP address for all components (default: $DEFAULT_IP):"
    read -r ip
    ip=${ip:-$DEFAULT_IP}
    echo "Enter the port number for all components (default: $DEFAULT_PORT):"
    read -r PORT
    PORT=${PORT:-$DEFAULT_PORT}
    SERVER_URL="https://$ip:$PORT"
    APP_URL="https://$ip:$PORT"
    CONTROLLER_URL="https://$ip:$PORT"
  fi

  # Validate inputs
  if [[ -z "$SERVER_URL" || -z "$APP_URL" || -z "$CONTROLLER_URL" ]]; then
    log_with_timestamp "Error: All URLs must be provided."
    exit 1
  fi
  if [[ ! "$SERVER_URL" =~ ^https:// || ! "$APP_URL" =~ ^https:// || ! "$CONTROLLER_URL" =~ ^https:// ]]; then
    log_with_timestamp "Error: All URLs must start with https://"
    exit 1
  fi
  if ! [[ "$PORT" =~ ^[0-9]+$ ]]; then
    log_with_timestamp "Error: Port must be a valid number"
    exit 1
  fi
}

build_project() {
  local project="$1"
  local mode="$2" # "debug" or "release"
  local target="$3"
  local output_dir="$4"
  local use_cross="$5" # "true" for cross-compilation, "false" for native

  log_with_timestamp "Building $project for target $target in $mode mode..."
  if [[ ! -d "$PROJECT_ROOT/$project" ]]; then
    log_with_timestamp "Error: Project directory $PROJECT_ROOT/$project does not exist"
    exit 1
  fi
  pushd "$PROJECT_ROOT/$project" >/dev/null || {
    log_with_timestamp "Error: Failed to enter $project directory"
    exit 1
  }

  local cargo_args="--target $target"
  if [[ "$mode" == "release" ]]; then
    cargo_args="$cargo_args --release"
  fi

  if [[ "$use_cross" == "true" ]]; then
    cross build $cargo_args 2>&1 | tee -a "$BUILD_LOG" || {
      log_with_timestamp "Error: Build failed for $project on $target"
      exit 1
    }
  else
    cargo build $cargo_args 2>&1 | tee -a "$BUILD_LOG" || {
      log_with_timestamp "Error: Build failed for $project on $target"
      exit 1
    }
  fi
  popd >/dev/null

  if [[ "$project" != "casaverde_utils" ]]; then
    # Copy binary to output directory
    local bin_name="casaverde_${project##casaverde_}"
    local bin_path="$PROJECT_ROOT/target/$target/$mode/$bin_name"
    if [[ "$target" == "x86_64-pc-windows-gnu" ]]; then
      bin_path="${bin_path}.exe"
      bin_name="${bin_name}.exe"
    fi
    if [[ -f "$bin_path" ]]; then
      cp "$bin_path" "$output_dir/$bin_name" || {
        log_with_timestamp "Error: Failed to copy $bin_name to $output_dir"
        exit 1
      }
      log_with_timestamp "Copied $bin_name to $output_dir"
    else
      log_with_timestamp "Error: Binary $bin_name not found at $bin_path"
      exit 1
    fi
  fi
  log_with_timestamp "Build complete: $project for $target in $mode mode"
}

setup_test_environment() {
  log_with_timestamp "Setting up test environment in $TESTING_ROOT..."
  mkdir -p "$TESTING_ROOT/casaverde_app" "$TESTING_ROOT/casaverde_controller" "$TESTING_ROOT/casaverde_server" || {
    log_with_timestamp "Error: Failed to create testing directories"
    exit 1
  }
  mkdir -p "$CONFIG_DIR" || {
    log_with_timestamp "Error: Failed to create $CONFIG_DIR"
    exit 1
  }

  # Generate config.toml for casaverde_server
  local server_config="$CONFIG_DIR/config.toml"
  cat >"$server_config" <<EOF
server = "$SERVER_URL"
EOF
  log_with_timestamp "Generated $server_config with server URL $SERVER_URL"

  # Copy Linux binaries to test environment (for local testing on Arch Linux)
  for project in "casaverde_app" "casaverde_controller" "casaverde_server"; do
    local bin_name="casaverde_${project##casaverde_}"
    local bin_path="$BUILD_OUTPUT/linux/$bin_name"
    local dest_dir="$TESTING_ROOT/$project"
    if [[ -f "$bin_path" ]]; then
      cp "$bin_path" "$dest_dir/$bin_name" || {
        log_with_timestamp "Error: Failed to copy $bin_name to $dest_dir"
        exit 1
      }
      log_with_timestamp "Copied $bin_name to $dest_dir"
    else
      log_with_timestamp "Error: Binary $bin_name not found at $bin_path"
      exit 1
    fi
  done

  # Generate or update config.toml for casaverde_app
  local app_config="$TESTING_ROOT/casaverde_app/config.toml"
  cat >"$app_config" <<EOF
server = "$APP_URL"

[[configs]]
id = "blackbeard-cpu"
type = "temperature"
endpoint = "/sensor_data"
interval = 15
serial_port = ""

[[configs]]
id = "solar-1"
type = "solar"
endpoint = "/sensor_data"
serial_port = ""
interval = 15

[[configs]]
id = "moisture-1"
type = "moisture"
endpoint = "/sensor_data"
serial_port = ""
interval = 15

[[configs]]
id = "humidity-1"
type = "humidity"
endpoint = "/sensor_data"
serial_port = ""
interval = 15

[[configs]]
id = "water-1"
type = "water"
endpoint = "/sensor_data"
serial_port = ""
interval = 15

[[configs]]
id = "relay-1"
type = "relay"
endpoint = "/relay_control"
serial_port = ""
interval = 15

[[configs]]
id = "blackbeard-probe"
type = "temperature"
endpoint = "/sensor_data"
serial_port = ""
interval = 15
EOF
  log_with_timestamp "Generated $app_config with server URL $APP_URL"

  # Generate or update config.toml for casaverde_controller
  local controller_config="$TESTING_ROOT/casaverde_controller/config.toml"
  cat >"$controller_config" <<EOF
server = "$CONTROLLER_URL"
controller_id = "blackbeard-pi"
serial_port = "/dev/ttyACM0"
light_relay_id = "relay-1"
light_on_hours = 16
light_off_hours = 8
EOF
  log_with_timestamp "Generated $controller_config with server URL $CONTROLLER_URL"

  # Generate self-signed TLS certificate
  local cert_path="$CONFIG_DIR/server.crt"
  local key_path="$CONFIG_DIR/server.key"
  if [[ ! -f "$cert_path" || ! -f "$key_path" ]]; then
    log_with_timestamp "Generating self-signed TLS certificate..."
    mkdir -p "$CONFIG_DIR" || {
      log_with_timestamp "Error: Failed to create $CONFIG_DIR"
      exit 1
    }
    openssl req -x509 -newkey rsa:4096 -keyout "$key_path" -out "$cert_path" \
      -sha256 -days 3650 -nodes -subj "/CN=casaverde.local" 2>&1 | tee -a "$BUILD_LOG" || {
      log_with_timestamp "Error: Failed to generate certificate"
      exit 1
    }
    log_with_timestamp "Certificate generated at $cert_path"
  fi
  for project in "casaverde_app" "casaverde_controller"; do
    local dest_cert="$TESTING_ROOT/$project/server.crt"
    cp "$cert_path" "$dest_cert" || {
      log_with_timestamp "Error: Failed to copy server.crt to $dest_cert"
      exit 1
    }
    log_with_timestamp "Copied server.crt to $dest_cert"
  done

  # Copy configs and certificates to platform-specific directories
  for platform in "linux" "windows" "raspberry_pi/arm" "raspberry_pi/arm64"; do
    local dest_dir="$BUILD_OUTPUT/$platform"
    cp "$app_config" "$dest_dir/casaverde_app_config.toml" || {
      log_with_timestamp "Error: Failed to copy app config to $dest_dir"
      exit 1
    }
    cp "$controller_config" "$dest_dir/casaverde_controller_config.toml" || {
      log_with_timestamp "Error: Failed to copy controller config to $dest_dir"
      exit 1
    }
    cp "$server_config" "$dest_dir/casaverde_server_config.toml" || {
      log_with_timestamp "Error: Failed to copy server config to $dest_dir"
      exit 1
    }
    cp "$cert_path" "$dest_dir/server.crt" || {
      log_with_timestamp "Error: Failed to copy server.crt to $dest_dir"
      exit 1
    }
    cp "$key_path" "$dest_dir/server.key" || {
      log_with_timestamp "Error: Failed to copy server.key to $dest_dir"
      exit 1
    }
    log_with_timestamp "Copied configuration files and certificates to $dest_dir"
  done

  log_with_timestamp "Test environment setup complete"
}

open_port() {
  log_with_timestamp "Opening port $PORT for casaverde_server..."
  case "$(uname -s)" in
  Linux*)
    if command -v ufw >/dev/null; then
      sudo ufw allow "$PORT/tcp" && sudo ufw reload 2>&1 | tee -a "$BUILD_LOG" || {
        log_with_timestamp "Error: Failed to open port $PORT with ufw"
        exit 1
      }
    else
      log_with_timestamp "ufw not found. Please manually open port $PORT:"
      log_with_timestamp "  sudo firewall-cmd --add-port=$PORT/tcp --permanent"
      log_with_timestamp "  sudo firewall-cmd --reload"
    fi
    ;;
  *)
    log_with_timestamp "Port opening not supported on this OS for testing. Please manually open port $PORT if needed."
    ;;
  esac
  log_with_timestamp "Port $PORT configuration complete"
}

main() {
  local action="${1:-}"
  if [[ -z "$action" ]]; then
    log_with_timestamp "Error: No action provided. Usage: $0 [debug | release | clean]"
    exit 1
  fi

  if [[ "$action" == "debug" || "$action" == "release" ]]; then
    prompt_config
  fi

  log_with_timestamp "Starting build process: $action with SERVER_URL=$SERVER_URL, APP_URL=$APP_URL, CONTROLLER_URL=$CONTROLLER_URL, PORT=$PORT"
  case "$action" in
  "debug")
    log_with_timestamp "Starting debug build process for all components..."
    # Build for Linux (native)
    for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
      build_project "$project" "debug" "${TARGETS[linux]}" "${BUILD_OUTPUT}/linux" "false"
    done
    # Cross-compile for Windows
    for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
      build_project "$project" "debug" "${TARGETS[windows]}" "${BUILD_OUTPUT}/windows" "true"
    done
    # Cross-compile for Raspberry Pi (ARM and ARM64)
    for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
      build_project "$project" "debug" "${TARGETS[raspberry_pi_arm]}" "${BUILD_OUTPUT}/raspberry_pi/arm" "true"
      build_project "$project" "debug" "${TARGETS[raspberry_pi_arm64]}" "${BUILD_OUTPUT}/raspberry_pi/arm64" "true"
    done
    setup_test_environment
    open_port
    log_with_timestamp "Debug build and test environment setup complete"
    echo "Do you want to deploy now? (y/n)"
    read -r deploy_now
    if [[ "$deploy_now" =~ ^[Yy]$ ]]; then
      if [[ -f "$PROJECT_ROOT/deploy.sh" ]]; then
        ./deploy.sh
      else
        log_with_timestamp "Warning: deploy.sh not found, skipping deployment"
      fi
    fi
    ;;
  "release")
    log_with_timestamp "Starting release build process for all components..."
    # Build for Linux (native)
    for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
      build_project "$project" "release" "${TARGETS[linux]}" "${BUILD_OUTPUT}/linux" "false"
    done
    # Cross-compile for Windows
    for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
      build_project "$project" "release" "${TARGETS[windows]}" "${BUILD_OUTPUT}/windows" "true"
    done
    # Cross-compile for Raspberry Pi (ARM and ARM64)
    for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
      build_project "$project" "release" "${TARGETS[raspberry_pi_arm]}" "${BUILD_OUTPUT}/raspberry_pi/arm" "true"
      build_project "$project" "release" "${TARGETS[raspberry_pi_arm64]}" "${BUILD_OUTPUT}/raspberry_pi/arm64" "true"
    done
    setup_test_environment
    if [[ -f "$PROJECT_ROOT/deploy.sh" ]]; then
      ./deploy.sh
    else
      log_with_timestamp "Warning: deploy.sh not found, skipping deployment"
    fi
    log_with_timestamp "Release build complete"
    ;;
  "clean")
    log_with_timestamp "Cleaning build artifacts..."
    cargo clean --manifest-path "$PROJECT_ROOT/Cargo.toml" 2>&1 | tee -a "$BUILD_LOG" || {
      log_with_timestamp "Error: Clean failed"
      exit 1
    }
    if [[ -d "$BUILD_OUTPUT" ]]; then
      rm -rf "$BUILD_OUTPUT" || {
        log_with_timestamp "Warning: Failed to remove $BUILD_OUTPUT, possibly due to permissions"
      }
    fi
    if [[ -d "$TESTING_ROOT" ]]; then
      rm -rf "$TESTING_ROOT" || {
        log_with_timestamp "Warning: Failed to remove $TESTING_ROOT, possibly due to permissions"
      }
    fi
    log_with_timestamp "Cleanup complete"
    ;;
  *)
    log_with_timestamp "Usage: $0 [debug | release | clean]"
    exit 1
    ;;
  esac
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi
