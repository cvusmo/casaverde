#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTING_ROOT="${PROJECT_ROOT}/casaverde_test"
CONFIG_DIR="${HOME}/.config/casaverde_server"
BUILD_LOG="${PROJECT_ROOT}/build.log"
DEFAULT_PORT="3003"

# Default values for URLs and ports
SERVER_URL=""
APP_URL=""
CONTROLLER_URL=""
PORT=""

# OS and architecture detection
OS="unknown"
ARCH="$(uname -m)"
case "$(uname -s)" in
    Linux*)
        if [[ "$ARCH" == "armv7l" || "$ARCH" == "armv6l" ]]; then
            OS="debian_arm"
        elif [[ "$ARCH" == "aarch64" ]]; then
            OS="debian_arm64"
        else
            OS="linux"
        fi
        ;;
    Darwin*)  OS="macos" ;;
    MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
esac

declare -A TARGETS
TARGETS["linux"]="x86_64-unknown-linux-gnu"
TARGETS["macos"]="x86_64-apple-darwin"
TARGETS["windows"]="x86_64-pc-windows-gnu"
TARGETS["debian_arm"]="armv7-unknown-linux-gnueabihf"
TARGETS["debian_arm64"]="aarch64-unknown-linux-gnu"

RUST_TARGET="${TARGETS[$OS]:-x86_64-unknown-linux-gnu}"
USE_TARGET_FLAG=false
if [[ "$OS" != "linux" || "$ARCH" != "x86_64" ]]; then
    USE_TARGET_FLAG=true
fi

# Use ~/.local/bin for all non-Windows OS to avoid permission issues
INSTALL_DIR="${HOME}/.local/bin"
[[ "$OS" == "windows" ]] && INSTALL_DIR="${HOME}/bin"
mkdir -p "$INSTALL_DIR"

log_with_timestamp() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$BUILD_LOG"
}

get_local_ip() {
    case "$OSTYPE" in
        "linux-gnu"*)
            ip -4 addr show | grep -oP '(?<=inet\s)\d+(\.\d+){3}' | grep -v "127.0.0.1" | head -n 1
            ;;
        "darwin"*)
            ifconfig | grep -oP 'inet\s+\K\d+(\.\d+){3}' | grep -v "127.0.0.1" | head -n 1
            ;;
        "msys"|"cygwin"|"win32")
            ipconfig | grep -oP 'IPv4 Address.*:\s*\K\d+(\.\d+){3}' | head -n 1
            ;;
        *)
            echo "127.0.0.1"
            ;;
    esac
}

prompt_config() {
    local default_ip=$(get_local_ip)
    echo "Do you want to install with default settings? (Port: $DEFAULT_PORT, Server: https://$default_ip, App: https://$default_ip, Controller: https://$default_ip) (y/n)"
    read -r use_defaults
    echo "Enter the port number for all components (default: $DEFAULT_PORT):"
    read -r PORT
    PORT=${PORT:-$DEFAULT_PORT}
    if [[ "$use_defaults" =~ ^[Yy]$ ]]; then
        SERVER_URL="https://$default_ip:$PORT"
        APP_URL="$SERVER_URL"
        CONTROLLER_URL="$SERVER_URL"
    else
        echo "Enter the IP address for casaverde_server (default: $default_ip):"
        read -r SERVER_IP
        SERVER_IP=${SERVER_IP:-$default_ip}
        SERVER_URL="https://$SERVER_IP:$PORT"
        echo "Enter the IP address for casaverde_app (default: $default_ip):"
        read -r APP_IP
        APP_IP=${APP_IP:-$default_ip}
        APP_URL="https://$APP_IP:$PORT"
        echo "Enter the IP address for casaverde_controller (default: $default_ip):"
        read -r CONTROLLER_IP
        CONTROLLER_IP=${CONTROLLER_IP:-$default_ip}
        CONTROLLER_URL="https://$CONTROLLER_IP:$PORT"
    fi

    # Validate inputs
    if [[ -z "$SERVER_URL" || -z "$APP_URL" || -z "$CONTROLLER_URL" ]]; then
        log_with_timestamp "Error: All URLs must be provided."
        exit 1
    fi
    if [[ -z "$PORT" ]]; then
        log_with_timestamp "Error: Port must be provided."
        exit 1
    fi
    if ! [[ "$SERVER_URL" =~ ^https:// && "$APP_URL" =~ ^https:// && "$CONTROLLER_URL" =~ ^https:// ]]; then
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
    local mode="$2"  # "debug" or "release"
    log_with_timestamp "Building $project for $OS ($RUST_TARGET) in $mode mode..."
    if [[ ! -d "$PROJECT_ROOT/$project" ]]; then
        log_with_timestamp "Error: Project directory $PROJECT_ROOT/$project does not exist"
        exit 1
    fi
    pushd "$PROJECT_ROOT/$project" >/dev/null || { log_with_timestamp "Error: Failed to enter $project directory"; exit 1; }
    local cargo_args=""
    if [[ "$mode" == "release" ]]; then
        cargo_args="--release"
    fi
    if [[ "$USE_TARGET_FLAG" == true ]]; then
        cargo build $cargo_args --target "$RUST_TARGET" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Error: Build failed for $project"; exit 1; }
    else
        cargo build $cargo_args 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Error: Build failed for $project"; exit 1; }
    fi
    popd >/dev/null
    log_with_timestamp "Build complete: $project in $mode mode"
}

install_binary() {
    local project="$1"
    local mode="$2"  # "debug" or "release"
    local bin_name="casaverde_${project##casaverde_}"
    local bin_path
    local workspace_bin_path="$PROJECT_ROOT/target/$mode/$bin_name"
    if [[ "$USE_TARGET_FLAG" == true ]]; then
        bin_path="$PROJECT_ROOT/target/$RUST_TARGET/$mode/$bin_name"
    else
        bin_path="$PROJECT_ROOT/$project/target/$mode/$bin_name"
        if [[ ! -f "$bin_path" ]]; then
            bin_path="$workspace_bin_path"
        fi
    fi
    log_with_timestamp "Installing $project binary to $INSTALL_DIR"
    if [[ ! -f "$bin_path" ]]; then
        log_with_timestamp "Error: Binary not found at $bin_path or $workspace_bin_path"
        exit 1
    fi
    if [[ "$OS" == "windows" ]]; then
        cp "$bin_path.exe" "$INSTALL_DIR/$bin_name.exe" || { log_with_timestamp "Error: Failed to install $bin_name"; exit 1; }
    else
        cp "$bin_path" "$INSTALL_DIR/$bin_name" || { log_with_timestamp "Error: Failed to install $bin_name"; exit 1; }
    fi
    log_with_timestamp "Installed $bin_name to $INSTALL_DIR"
}

setup_test_environment() {
    log_with_timestamp "Setting up test environment in $TESTING_ROOT..."
    mkdir -p "$TESTING_ROOT/casaverde_app" "$TESTING_ROOT/casaverde_controller" "$TESTING_ROOT/casaverde_server" || {
        log_with_timestamp "Error: Failed to create testing directories"
        exit 1
    }
    mkdir -p "$CONFIG_DIR" || { log_with_timestamp "Error: Failed to create $CONFIG_DIR"; exit 1; }

    # Generate config.toml for casaverde_server
    local server_config="$CONFIG_DIR/config.toml"
    cat > "$server_config" << EOF
server = "$SERVER_URL"
EOF
    log_with_timestamp "Generated $server_config with server URL $SERVER_URL"

    for project in "casaverde_app" "casaverde_controller" "casaverde_server"; do
        local bin_name="casaverde_${project##casaverde_}"
        local bin_path
        if [[ "$USE_TARGET_FLAG" == true ]]; then
            bin_path="$PROJECT_ROOT/target/$RUST_TARGET/debug/$bin_name"
        else
            bin_path="$PROJECT_ROOT/target/debug/$bin_name"
        fi
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
    cat > "$app_config" << EOF
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
    cat > "$controller_config" << EOF
server = "$CONTROLLER_URL"
controller_id = "blackbeard-pi"
serial_port = "/dev/ttyACM0"
light_relay_id = "relay-1"
light_on_hours = 16
light_off_hours = 8
EOF
    log_with_timestamp "Generated $controller_config with server URL $CONTROLLER_URL"

    local cert_path="$CONFIG_DIR/server.crt"
    local key_path="$CONFIG_DIR/server.key"
    if [[ ! -f "$cert_path" || ! -f "$key_path" ]]; then
        log_with_timestamp "Generating self-signed TLS certificate..."
        mkdir -p "$CONFIG_DIR" || { log_with_timestamp "Error: Failed to create $CONFIG_DIR"; exit 1; }
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
    log_with_timestamp "Test environment setup complete"
}

open_port() {
    log_with_timestamp "Opening port $PORT for casaverde_server..."
    case "$OSTYPE" in
        "linux-gnu"*)
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
        "darwin"*)
            log_with_timestamp "Please manually open port $PORT on macOS:"
            log_with_timestamp "  sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add-port $PORT"
            ;;
        "msys"|"cygwin"|"win32")
            log_with_timestamp "Please manually open port $PORT on Windows:"
            log_with_timestamp "  netsh advfirewall firewall add rule name=\"casaverde_server\" dir=in action=allow protocol=TCP localport=$PORT"
            ;;
        *)
            log_with_timestamp "Unsupported OS for automatic port configuration: $OSTYPE"
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

    # debug or release actions
    if [[ "$action" == "debug" || "$action" == "release" ]]; then
        prompt_config
    fi

    log_with_timestamp "Starting build process: $action with SERVER_URL=$SERVER_URL, APP_URL=$APP_URL, CONTROLLER_URL=$CONTROLLER_URL, PORT=$PORT"
    case "$action" in
        "debug")
            log_with_timestamp "Starting debug build process for all components..."
            for project in "casaverde_utils" "casaverde_server" "casaverde_app" "casaverde_controller"; do
                build_project "$project" "debug"
            done
            for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
                install_binary "$project" "debug"
            done
            setup_test_environment
            open_port
            log_with_timestamp "Debug build, installation, and test environment setup complete for $OS"
            echo "Do you want to deploy now? (y/n)"
            read -r deploy_now
            if [[ "$deploy_now" =~ ^[Yy]$ ]]; then
                ./deploy.sh
            fi
            ;;
        "release")
            log_with_timestamp "Starting release build process for all components..."
            for project in "casaverde_utils" "casaverde_server" "casaverde_app" "casaverde_controller"; do
                build_project "$project" "release"
            done
            for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
                install_binary "$project" "release"
            done
            log_with_timestamp "Release build and installation complete for $OS"
            ./deploy.sh
            ;;
        "clean")
            log_with_timestamp "Cleaning build artifacts..."
            cargo clean --manifest-path "$PROJECT_ROOT/Cargo.toml" 2>&1 | tee -a "$BUILD_LOG" || {
                log_with_timestamp "Error: Clean failed"
                exit 1
            }
            if [[ -d "$TESTING_ROOT" ]]; then
                if ! rm -rf "$TESTING_ROOT" 2>/dev/null; then
                    log_with_timestamp "Warning: Failed to remove $TESTING_ROOT, possibly due to permissions or usage. Please remove manually: sudo rm -rf $TESTING_ROOT"
                else
                    log_with_timestamp "Removed $TESTING_ROOT"
                fi
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

