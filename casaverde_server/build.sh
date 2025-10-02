#!/usr/bin/env bash
# Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
# Build and install script for casaverde_server

set -euo pipefail

APP_NAME="casaverde_server"
APP_BIN="target/release/$APP_NAME"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/casaverde_server"
CERT_DIR="$HOME/.casaverde_cert"

SERVER_KEY="$CONFIG_DIR/server.key"
SERVER_CRT="$CONFIG_DIR/server.crt"

echo "🔨 Building $APP_NAME..."
cargo build --release

echo "📂 Creating config + cert directories..."
mkdir -p "$CONFIG_DIR"
mkdir -p "$CERT_DIR"

echo "📦 Installing binary to $INSTALL_DIR..."
sudo cp "$APP_BIN" "$INSTALL_DIR/$APP_NAME"

if [ ! -f "$SERVER_KEY" ] || [ ! -f "$SERVER_CRT" ]; then
    echo "🔑 Generating new self-signed TLS certificate..."
    openssl req -x509 -newkey rsa:4096 -keyout "$SERVER_KEY" -out "$SERVER_CRT" -sha256 -days 3650 -nodes -subj "/CN=casaverde.local"
    echo "✅ Certificate generated at:"
    echo "   $SERVER_KEY"
    echo "   $SERVER_CRT"
else
    echo "✅ Existing server certificate found at $CONFIG_DIR"
fi

echo "📤 Exporting server.crt to $CERT_DIR..."
cp "$SERVER_CRT" "$CERT_DIR/server.crt"

echo "🎉 $APP_NAME build and install complete!"
echo "👉 Run with: $APP_NAME"

