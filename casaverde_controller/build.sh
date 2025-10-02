#!/usr/bin/env bash
# Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
# Build and install script for casaverde_controller

set -euo pipefail

APP_NAME="casaverde_controller"
APP_BIN="target/release/$APP_NAME"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/casaverde_controller"
CERT_DIR="$HOME/.casaverde_cert"
DEFAULT_CONFIG="config.toml"

echo "🔨 Building $APP_NAME..."
cargo build --release

echo "📂 Creating config + cert directories..."
mkdir -p "$CONFIG_DIR"
mkdir -p "$CERT_DIR"

echo "📦 Installing binary to $INSTALL_DIR..."
sudo cp "$APP_BIN" "$INSTALL_DIR/$APP_NAME"

if [ ! -f "$CONFIG_DIR/$DEFAULT_CONFIG" ]; then
    echo "📝 Copying default config.toml to $CONFIG_DIR"
    cp "$DEFAULT_CONFIG" "$CONFIG_DIR/$DEFAULT_CONFIG" || {
        echo "⚠️  No local config.toml found, please create one manually at $CONFIG_DIR/$DEFAULT_CONFIG"
    }
else
    echo "✅ Config already exists at $CONFIG_DIR/$DEFAULT_CONFIG"
fi

if [ -f "server.crt" ]; then
    echo "🔑 Installing server.crt to $CERT_DIR"
    cp "server.crt" "$CERT_DIR/server.crt"
elif [ -f "$CONFIG_DIR/server.crt" ]; then
    echo "🔑 Installing existing server.crt from config dir to $CERT_DIR"
    cp "$CONFIG_DIR/server.crt" "$CERT_DIR/server.crt"
else
    echo "⚠️  server.crt not found. Please copy it manually to $CERT_DIR/server.crt"
fi

echo "🎉 $APP_NAME build and install complete!"
echo "👉 Run with: $APP_NAME"

