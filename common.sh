#!/usr/bin/env bash
# Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
# Shared build helper for Casaverde

set -euo pipefail

# --- Detect OS ---
OS="unknown"
case "$(uname -s)" in
    Linux*)   OS="linux" ;;
    Darwin*)  OS="macos" ;; 
    MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
esac

# --- Target triples ---
if [[ "$OS" == "linux" ]]; then
    RUST_TARGET="x86_64-unknown-linux-gnu"
    INSTALL_DIR="/usr/local/bin"
elif [[ "$OS" == "windows" ]]; then
    RUST_TARGET="x86_64-pc-windows-gnu"
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
elif [[ "$OS" == "macos" ]]; then
    RUST_TARGET="x86_64-apple-darwin"
    INSTALL_DIR="/usr/local/bin"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# --- Helpers ---
build_project() {
    local project="$1"
    echo "🔨 Building $project for $OS ($RUST_TARGET)..."
    pushd "$project" >/dev/null
    cargo build --release --target "$RUST_TARGET"
    popd >/dev/null
    echo "✅ Build complete: $project"
}

install_binary() {
    local project="$1"
    local bin_path="$project/target/$RUST_TARGET/release/$project"
    echo "📦 Installing $project binary to $INSTALL_DIR"
    if [[ ! -f "$bin_path" ]]; then
        echo "❌ Error: binary not found at $bin_path"
        exit 1
    fi
    if [[ "$OS" == "linux" || "$OS" == "macos" ]]; then
        sudo cp "$bin_path" "$INSTALL_DIR/$project"
    else
        cp "$bin_path" "$INSTALL_DIR/$project.exe"
    fi
}

setup_project_env() {
    local project="$1"
    echo "⚙️  Setting up environment for $project..."
    CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/$project"
    CERT_DIR="$HOME/.casaverde_cert"
    mkdir -p "$CONFIG_DIR" "$CERT_DIR"
}

ensure_config() {
    local project="$1"
    local default="$2"
    local config_path="${XDG_CONFIG_HOME:-$HOME/.config}/$project/$default"
    if [[ ! -f "$config_path" ]]; then
        if [[ -f "$project/$default" ]]; then
            echo "📝 Installing default $default to $config_path"
            cp "$project/$default" "$config_path"
        else
            echo "⚠️ No $default found for $project. Please create manually at $config_path"
        fi
    else
        echo "✅ Config already exists at $config_path"
    fi
}

ensure_certificates() {
    local project="$1"
    local config_dir="${XDG_CONFIG_HOME:-$HOME/.config}/$project"
    local cert_dir="$HOME/.casaverde_cert"
    local key="$config_dir/server.key"
    local crt="$config_dir/server.crt"

    if [[ ! -f "$key" || ! -f "$crt" ]]; then
        echo "🔑 Generating self-signed TLS certificate..."
        openssl req -x509 -newkey rsa:4096 -keyout "$key" -out "$crt" \
            -sha256 -days 3650 -nodes -subj "/CN=casaverde.local"
        echo "✅ Certificate generated at $crt"
    else
        echo "✅ Existing certs found in $config_dir"
    fi
    cp "$crt" "$cert_dir/server.crt"
}
