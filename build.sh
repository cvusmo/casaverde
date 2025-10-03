#!/usr/bin/env bash
# Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
# build.sh - Unified build, setup, and deployment script for Casaverde

set -euo pipefail

# --- Configuration ---
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYTHON_SCRIPT="${PROJECT_ROOT}/casaverde_automate.py"
TESTING_ROOT="${PROJECT_ROOT}/casaverde_test"
VENV_PYTHON="${PROJECT_ROOT}/casaverde_sim/venv/bin/python"
SIM_SCRIPT="${PROJECT_ROOT}/casaverde_sim/casaverde_sim.py"
CONFIG_DIR="${HOME}/.config/casaverde_server"
BUILD_LOG="${PROJECT_ROOT}/build.log"

# --- Target Triples and OS Detection ---
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

# Use native build (no --target) unless cross-compiling is needed
RUST_TARGET="${TARGETS[$OS]:-x86_64-unknown-linux-gnu}" # Default to Linux if unsupported
USE_TARGET_FLAG=false
if [[ "$OS" == "linux" && "$(uname -m)" == "x86_64" ]]; then
    USE_TARGET_FLAG=false # Native build, no need for --target
else
    USE_TARGET_FLAG=true
fi

INSTALL_DIR="/usr/local/bin"
[[ "$OS" == "windows" ]] && INSTALL_DIR="${HOME}/bin" && mkdir -p "$INSTALL_DIR"
[[ "$OS" == "macos" ]] && INSTALL_DIR="/usr/local/bin"

# --- Timing Helper ---
get_time_s() {
    date +%s.%N
}

log_with_timestamp() {
    local msg="$1"
    echo "[$(date '+%Y-%m-%d %H:%M:%S.%3N')] $msg" | tee -a "$BUILD_LOG"
}

# --- Helpers ---
build_project() {
    local project="$1"
    local start_time=$(get_time_s)
    log_with_timestamp "🔨 Building $project for $OS..."
    if [[ ! -d "$PROJECT_ROOT/$project" ]]; then
        log_with_timestamp "❌ Error: Project directory $PROJECT_ROOT/$project does not exist"
        exit 1
    fi
    pushd "$PROJECT_ROOT/$project" >/dev/null || { log_with_timestamp "❌ Failed to enter $project directory"; exit 1; }
    
    log_with_timestamp "DEBUG: Current directory: $(pwd)"
    [[ -f "Cargo.toml" ]] && log_with_timestamp "DEBUG: Cargo.toml found" || log_with_timestamp "DEBUG: Cargo.toml not found"

    if [[ "$USE_TARGET_FLAG" == true ]]; then
        cargo build --release --target "$RUST_TARGET" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "❌ Build failed for $project"; exit 1; }
    else
        cargo build --release 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "❌ Build failed for $project"; exit 1; }
    fi
    popd >/dev/null || exit 1

    local release_dir="$PROJECT_ROOT/$project/target/release"
    if [[ "$USE_TARGET_FLAG" == true ]]; then
        release_dir="$PROJECT_ROOT/$project/target/$RUST_TARGET/release"
    fi
    log_with_timestamp "DEBUG: Checking for $release_dir"
    if [[ -d "$release_dir" ]]; then
        log_with_timestamp "DEBUG: Contents of $release_dir:"
        ls -l "$release_dir" 2>&1 | tee -a "$BUILD_LOG"
    else
        log_with_timestamp "DEBUG: Directory $release_dir does not exist"
    fi
    local end_time=$(get_time_s)
    local duration=$(echo "$end_time - $start_time" | bc)
    log_with_timestamp "✅ Build complete: $project (Duration: $(printf "%.2f" $duration)s)"
}

install_binary() {
    local project="$1"
    local start_time=$(get_time_s)
    local bin_name="casaverde_${project##casaverde_}"
    local bin_path
    if [[ "$USE_TARGET_FLAG" == true ]]; then
        bin_path="$PROJECT_ROOT/$project/target/$RUST_TARGET/release/$bin_name"
    else
        bin_path="$PROJECT_ROOT/$project/target/release/$bin_name"
    fi
    local workspace_bin_path="$PROJECT_ROOT/target/release/$bin_name"
    log_with_timestamp "📦 Installing $project binary to $INSTALL_DIR"
    log_with_timestamp "DEBUG: Checking for binary at $bin_path"
    if [[ ! -f "$bin_path" ]]; then
        log_with_timestamp "DEBUG: Binary not found at $bin_path, checking workspace root: $workspace_bin_path"
        if [[ -f "$workspace_bin_path" ]]; then
            bin_path="$workspace_bin_path"
            log_with_timestamp "DEBUG: Found binary at $workspace_bin_path"
        else
            log_with_timestamp "DEBUG: Contents of $PROJECT_ROOT/$project/target/release (if exists):"
            ls -l "$PROJECT_ROOT/$project/target/release" 2>&1 | tee -a "$BUILD_LOG" || log_with_timestamp "DEBUG: Failed to list $PROJECT_ROOT/$project/target/release"
            log_with_timestamp "DEBUG: Contents of $PROJECT_ROOT/target/release (if exists):"
            ls -l "$PROJECT_ROOT/target/release" 2>&1 | tee -a "$BUILD_LOG" || log_with_timestamp "DEBUG: Failed to list $PROJECT_ROOT/target/release"
            log_with_timestamp "❌ Error: binary not found at $bin_path or $workspace_bin_path"
            exit 1
        fi
    fi
    if [[ "$OS" == "windows" ]]; then
        cp "$bin_path.exe" "$INSTALL_DIR/$bin_name.exe" || { log_with_timestamp "❌ Failed to install $bin_name"; exit 1; }
    else
        sudo cp "$bin_path" "$INSTALL_DIR/$bin_name" || { log_with_timestamp "❌ Failed to install $bin_name"; exit 1; }
    fi
    local end_time=$(get_time_s)
    local duration=$(echo "$end_time - $start_time" | bc)
    log_with_timestamp "✅ Installed $bin_name to $INSTALL_DIR (Duration: $(printf "%.2f" $duration)s)"
}

setup_project_env() {
    local project="$1"
    local start_time=$(get_time_s)
    log_with_timestamp "⚙️ Setting up environment for $project..."
    local config_dir="${XDG_CONFIG_HOME:-$HOME/.config}/$project"
    local cert_dir="$HOME/.casaverde_cert"
    mkdir -p "$config_dir" "$cert_dir" || { log_with_timestamp "❌ Failed to create directories for $project"; exit 1; }
    # Copy config.toml to ~/.config/$project/ for casaverde_app and casaverde_controller
    if [[ "$project" != "casaverde_server" ]]; then
        local config_path="$PROJECT_ROOT/$project/config.toml"
        local dest_config="$config_dir/config.toml"
        if [[ -f "$config_path" ]]; then
            cp "$config_path" "$dest_config" || {
                log_with_timestamp "❌ Failed to copy config.toml to $dest_config"
                exit 1
            }
            log_with_timestamp "✅ Copied config.toml to $dest_config"
        else
            log_with_timestamp "⚠️ No config.toml found for $project at $config_path"
        fi
    fi
    local end_time=$(get_time_s)
    local duration=$(echo "$end_time - $start_time" | bc)
    log_with_timestamp "✅ Environment setup complete for $project (Duration: $(printf "%.2f" $duration)s)"
}

ensure_config() {
    local project="$1"
    local default="$2"
    local start_time=$(get_time_s)
    if [[ "$project" == "casaverde_server" ]]; then
        log_with_timestamp "ℹ️ Skipping config check for $project (no config.toml required)"
        return
    fi
    local config_path="${XDG_CONFIG_HOME:-$HOME/.config}/$project/$default"
    if [[ ! -f "$config_path" ]]; then
        if [[ -f "$PROJECT_ROOT/$project/$default" ]]; then
            log_with_timestamp "📝 Installing default $default to $config_path"
            cp "$PROJECT_ROOT/$project/$default" "$config_path" || { log_with_timestamp "❌ Failed to copy $default"; exit 1; }
        else
            log_with_timestamp "⚠️ No $default found for $project. Please create manually at $config_path"
        fi
    else
        log_with_timestamp "✅ Config already exists at $config_path"
    fi
    local end_time=$(get_time_s)
    local duration=$(echo "$end_time - $start_time" | bc)
    log_with_timestamp "✅ Config check complete for $project (Duration: $(printf "%.2f" $duration)s)"
}

ensure_certificates() {
    local project="$1"
    local start_time=$(get_time_s)
    local config_dir="${XDG_CONFIG_HOME:-$HOME/.config}/$project"
    local cert_dir="$HOME/.casaverde_cert"
    local key="$config_dir/server.key"
    local crt="$config_dir/server.crt"

    if [[ ! -f "$key" || ! -f "$crt" ]]; then
        log_with_timestamp "🔑 Generating self-signed TLS certificate..."
        openssl req -x509 -newkey rsa:4096 -keyout "$key" -out "$crt" \
            -sha256 -days 3650 -nodes -subj "/CN=casaverde.local" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "❌ Failed to generate certificate"; exit 1; }
        log_with_timestamp "✅ Certificate generated at $crt"
    else
        log_with_timestamp "✅ Existing certs found in $config_dir"
    fi
    cp "$crt" "$cert_dir/server.crt" || { log_with_timestamp "❌ Failed to copy certificate to $cert_dir"; exit 1; }
    local end_time=$(get_time_s)
    local duration=$(echo "$end_time - $start_time" | bc)
    log_with_timestamp "✅ Certificate check complete for $project (Duration: $(printf "%.2f" $duration)s)"
}

copy_to_testing_root() {
    local start_time=$(get_time_s)
    log_with_timestamp "📂 Copying files to testing root: $TESTING_ROOT..."

    # Create testing root and subdirectories
    mkdir -p "$TESTING_ROOT/casaverde_app" "$TESTING_ROOT/casaverde_controller" "$TESTING_ROOT/casaverde_server" || {
        log_with_timestamp "❌ Failed to create testing directories"
        exit 1
    }

    # Copy binaries from workspace root
    for project in "casaverde_app" "casaverde_controller" "casaverde_server"; do
        local bin_name="casaverde_${project##casaverde_}"
        local bin_path="$PROJECT_ROOT/target/release/$bin_name"
        local dest_dir="$TESTING_ROOT/$project"
        if [[ -f "$bin_path" ]]; then
            cp "$bin_path" "$dest_dir/$bin_name" || {
                log_with_timestamp "❌ Failed to copy $bin_name to $dest_dir"
                exit 1
            }
            log_with_timestamp "✅ Copied $bin_name to $dest_dir"
        else
            log_with_timestamp "❌ Binary $bin_name not found at $bin_path"
            exit 1
        fi
    done

    # Copy config.toml for casaverde_app and casaverde_controller
    for project in "casaverde_app" "casaverde_controller"; do
        local config_path="$PROJECT_ROOT/$project/config.toml"
        local dest_config="$TESTING_ROOT/$project/config.toml"
        if [[ -f "$config_path" ]]; then
            cp "$config_path" "$dest_config" || {
                log_with_timestamp "❌ Failed to copy config.toml to $dest_config"
                exit 1
            }
            log_with_timestamp "✅ Copied config.toml to $dest_config"
        else
            log_with_timestamp "⚠️ No config.toml found for $project at $config_path"
        fi
    done

    # Copy server.crt for casaverde_app and casaverde_controller
    local cert_path="$CONFIG_DIR/server.crt"
    for project in "casaverde_app" "casaverde_controller"; do
        local dest_cert="$TESTING_ROOT/$project/server.crt"
        if [[ -f "$cert_path" ]]; then
            cp "$cert_path" "$dest_cert" || {
                log_with_timestamp "❌ Failed to copy server.crt to $dest_cert"
                exit 1
            }
            log_with_timestamp "✅ Copied server.crt to $dest_cert"
        else
            log_with_timestamp "❌ server.crt not found at $cert_path"
            exit 1
        fi
    done

    local end_time=$(get_time_s)
    local duration=$(echo "$end_time - $start_time" | bc)
    log_with_timestamp "✅ Files copied to testing root (Duration: $(printf "%.2f" $duration)s)"
}

open_port_3003() {
    local start_time=$(get_time_s)
    log_with_timestamp "🔓 Opening port 3003..."
    case "$OSTYPE" in
        "linux-gnu"*)
            if command -v ufw >/dev/null; then
                sudo ufw allow 3003/tcp && sudo ufw reload 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "❌ Failed to open port 3003 with ufw"; exit 1; }
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
    local end_time=$(get_time_s)
    local duration=$(echo "$end_time - $start_time" | bc)
    log_with_timestamp "✅ Port 3003 configuration complete (Duration: $(printf "%.2f" $duration)s)"
}

# --- Main Build and Deploy Logic ---
main() {
    local action="${1:-build}" # Default to build if no arg provided
    # Initialize build log
    local start_time=$(get_time_s)
    log_with_timestamp "📜 Starting build process"
    case "$action" in
        "build")
            log_with_timestamp "🏗 Starting build process for all components..."
            # Build all projects in the specified order
            for project in "casaverde_utils" "casaverde_server" "casaverde_app" "casaverde_controller"; do
                build_project "$project"
            done
            # Install binaries and setup environment
            for project in "casaverde_server" "casaverde_app" "casaverde_controller"; do
                install_binary "$project"
                setup_project_env "$project"
                ensure_config "$project" "config.toml"
            done
            ensure_certificates "casaverde_server"
            copy_to_testing_root
            open_port_3003
            local end_time=$(get_time_s)
            local duration=$(echo "$end_time - $start_time" | bc)
            log_with_timestamp "🎉 Build and installation complete for $OS! (Duration: $(printf "%.2f" $duration)s)"
            ;;
        "test")
            local test_start_time=$(get_time_s)
            log_with_timestamp "🧪 Starting test deployment..."
            if [[ -f "$PYTHON_SCRIPT" ]]; then
                python3 "$PYTHON_SCRIPT" --testing-root "$TESTING_ROOT" --project-root "$PROJECT_ROOT" \
                    --venv-python "$VENV_PYTHON" --sim-script "$SIM_SCRIPT" --config-dir "$CONFIG_DIR" \
                    "${@:2}" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "❌ Test deployment failed"; exit 1; }
            else
                log_with_timestamp "❌ Python automation script not found at $PYTHON_SCRIPT"
                exit 1
            fi
            local test_end_time=$(get_time_s)
            local test_duration=$(echo "$test_end_time - $test_start_time" | bc)
            log_with_timestamp "✅ Test deployment complete (Duration: $(printf "%.2f" $test_duration)s)"
            ;;
        "clean")
            local clean_start_time=$(get_time_s)
            log_with_timestamp "🧹 Cleaning build artifacts..."
            cargo clean --manifest-path "$PROJECT_ROOT/Cargo.toml" 2>&1 | tee -a "$BUILD_LOG" || { log_with_timestamp "❌ Clean failed"; exit 1; }
            [[ -d "$TESTING_ROOT" ]] && rm -rf "$TESTING_ROOT" || { log_with_timestamp "❌ Failed to remove $TESTING_ROOT"; exit 1; }
            local clean_end_time=$(get_time_s)
            local clean_duration=$(echo "$clean_end_time - $clean_start_time" | bc)
            log_with_timestamp "✅ Cleanup complete! (Duration: $(printf "%.2f" $clean_duration)s)"
            ;;
        *)
            log_with_timestamp "Usage: $0 {build|test|clean} [test_args]"
            exit 1
            ;;
    esac
}

# --- Execute ---
main "$@"
