#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTING_ROOT="${PROJECT_ROOT}/casaverde_test"
CONFIG_DIR="${HOME}/.config/casaverde_server"
BUILD_LOG="${PROJECT_ROOT}/build.log"

OS="unknown"
case "$(uname -s)" in
    Linux*)   OS="linux" ;;
    Darwin*)  OS="macos" ;;
    MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
    *armv7l*) OS="raspbian" ;;
esac

declare -A TARGETS
TARGETS["linux"]="x86_64-unknown-linux-gnu"
TARGETS["macos"]="x86_64-apple-darwin"
TARGETS["windows"]="x86_64-pc-windows-gnu"
TARGETS["raspbian"]="armv7-unknown-linux-gnueabihf"

RUST_TARGET="${TARGETS[$OS]:-x86_64-unknown-linux-gnu}"
USE_TARGET_FLAG=false
if [[ "$OS" != "linux" || "$(uname -m)" != "x86_64" ]]; then
    USE_TARGET_FLAG=true
fi

INSTALL_DIR="/usr/local/bin"
[[ "$OS" == "windows" ]] && INSTALL_DIR="${HOME}/bin" && mkdir -p "$INSTALL_DIR"
[[ "$OS" == "macos" ]] && INSTALL_DIR="/usr/local/bin"

log_with_timestamp() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$BUILD_LOG"
}

build_project() {
    local project="$1"
    log_with_timestamp "Building $project for $OS..."
    if [[ ! -d "$PROJECT_ROOT/$project" ]]; then
        log_with_timestamp "Error: Project directory $PROJECT_ROOT/$project does not exist"
        exit 1
    fi
    pushd "$PROJECT_ROOT/$project" >/dev/null || { log_with_timestamp "Error: Failed to enter $project directory"; exit 1; }
    if [[ "$USE_TARGET_FLAG" == true ]]; then
        cargo build --release --target "$RUST_TARGET" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Error: Build failed for $project"; exit 1; }
    else
        cargo build --release 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "Error: Build failed for $project"; exit 1; }
    fi
    popd >/dev/null
    log_with_timestamp "Build complete: $project"
}

install_binary() {
    local project="$1"
    local bin_name="casaverde_${project##casaverde_}"
    local bin_path
    if [[ "$USE_TARGET_FLAG" == true ]]; then
        bin_path="$PROJECT_ROOT/$project/target/$RUST_TARGET/release/$bin_name"
    else
        bin_path="$PROJECT_ROOT/$project/target/release/$bin_name"
    fi
    local workspace_bin_path="$PROJECT_ROOT/target/release/$bin_name"
    log_with_timestamp "Installing $project binary to $INSTALL_DIR"
    if [[ ! -f "$bin_path" && -f "$workspace_bin_path" ]]; then
        bin_path="$workspace_bin_path"
    elif [[ ! -f "$bin_path" ]]; then
        log_with_timestamp "Error: Binary not found at $bin_path or $workspace_bin_path"
        exit 1
    fi
    if [[ "$OS" == "windows" ]]; then
        cp "$bin_path.exe" "$INSTALL_DIR/$bin_name.exe" || { log_with_timestamp "Error: Failed to install $bin_name"; exit 1; }
    else
        sudo cp "$bin_path" "$INSTALL_DIR/$bin_name" || { log_with_timestamp "Error: Failed to install $bin_name"; exit 1; }
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
    for project in "casaverde_app" "casaverde_controller" "casaverde_server"; do
        local bin_name="casaverde_${project##casaverde_}"
        local bin_path="$PROJECT_ROOT/target/release/$bin_name"
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
    for project in "casaverde_app" "casaverde_controller"; do
        local config_path="$PROJECT_ROOT/$project/config.toml"
        local dest_config="$TESTING_ROOT/$project/config.toml"
        if [[ -f "$config_path" ]]; then
            cp "$config_path" "$dest_config" || {
                log_with_timestamp "Error: Failed to copy config.toml to $dest_config"
                exit 1
            }
            log_with_timestamp "Copied config.toml to $dest_config"
        else
            log_with_timestamp "Warning: No config.toml found for $project at $config_path"
        fi
    done
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

open_port_3003() {
    log_with_timestamp "Opening port 3003..."
    case "$OSTYPE" in
        "linux-gnu"*)
            if command -v ufw >/dev/null; then
                sudo ufw allow 3003/tcp && sudo ufw reload 2>&1 | tee -a "$BUILD_LOG" || {
                    log_with_timestamp "Error: Failed to open port 3003 with ufw"
                    exit 1
                }
            else
                log_with_timestamp "ufw not found. Please manually open port 3003:"
                log_with_timestamp "  sudo firewall-cmd --add-port=3003/tcp --permanent"
                log_with_timestamp "  sudo firewall-cmd --reload"
            fi
            ;;
        "darwin"*)
            log_with_timestamp "Please manually open port 3003 on macOS:"
            log_with_timestamp "  sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add-port 3003"
            ;;
        "msys"|"cygwin"|"win32")
            log_with_timestamp "Please manually open port 3003 on Windows:"
            log_with_timestamp "  netsh advfirewall firewall add rule name=\"casaverde_server\" dir=in action=allow protocol=TCP localport=3003"
            ;;
        *)
            log_with_timestamp "Unsupported OS for automatic port configuration: $OSTYPE"
            ;;
    esac
    log_with_timestamp "Port 3003 configuration complete"
}

main() {
    local action="${1:-build}"
    log_with_timestamp "Starting build process: $action"
    case "$action" in
        "build")
            log_with_timestamp "Starting build process for all components..."
            for project in "casaverde_utils" "casaverde_server" "casaverde_app" "casaverde_controller"; do
                build_project "$project"
            done
            for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
                install_binary "$project"
            done
            log_with_timestamp "Build and installation complete for $OS"
            ;;
        "test")
            log_with_timestamp "Starting test environment build..."
            for project in "casaverde_utils" "casaverde_server" "casaverde_app" "casaverde_controller"; do
                build_project "$project"
            done
            setup_test_environment
            open_port_3003
            log_with_timestamp "Test environment build complete"
            ;;
        "clean")
            log_with_timestamp "Cleaning build artifacts..."
            cargo clean --manifest-path "$PROJECT_ROOT/Cargo.toml" 2>&1 | tee -a "$BUILD_LOG" || {
                log_with_timestamp "Error: Clean failed"
                exit 1
            }
            [[ -d "$TESTING_ROOT" ]] && rm -rf "$TESTING_ROOT" || {
                log_with_timestamp "Error: Failed to remove $TESTING_ROOT"
                exit 1
            }
            log_with_timestamp "Cleanup complete"
            ;;
        *)
            log_with_timestamp "Usage: $0 [build | test | clean]"
            exit 1
            ;;
    esac
}

main "$@"
