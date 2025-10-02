#!/usr/bin/env bash
# Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
# Build and install script for casaverde_app

set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
source "$ROOT_DIR/common.sh"

APP_NAME="casaverde_app"

setup_project_env "$APP_NAME"
build_project "$APP_NAME"
install_binary "$APP_NAME"
ensure_config "$APP_NAME" "config.toml"

echo "🎉 $APP_NAME build and install complete!"
echo "👉 Run with: $APP_NAME --tui"
