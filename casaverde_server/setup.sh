#!/bin/bash

# setup.sh for casaverde_server
# Run on server to set up certificates and firewall
# Use --test to print the detected IP address and exit

set -e

# Check for --test flag
if [[ "$1" == "--test" ]]; then
    # Get OS-specific IP detection
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v ip >/dev/null; then
            SERVER_IP=$(ip addr show | grep -o "inet [0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}" | grep -v "inet 127\." | head -n 1 | cut -d' ' -f2)
        elif command -v ifconfig >/dev/null; then
            SERVER_IP=$(ifconfig | grep -o "inet [0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}" | grep -v "inet 127\." | head -n 1 | cut -d' ' -f2)
        else
            SERVER_IP=""
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v ifconfig >/dev/null; then
            SERVER_IP=$(ifconfig | grep -o "inet [0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}" | grep -v "inet 127\." | head -n 1 | cut -d' ' -f2)
        else
            SERVER_IP=""
        fi
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
        if command -v ipconfig >/dev/null; then
            SERVER_IP=$(ipconfig | grep -o "IPv4 Address.*[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}" | grep -v "127\." | head -n 1 | grep -o "[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}")
        else
            SERVER_IP=""
        fi
    else
        SERVER_IP=""
    fi

    # Default to 10.0.0.12:3003 if no IP found or use environment variable
    SERVER_IP="${SERVER_IP:-10.0.0.12}:3003"

    echo "Detected server IP: $SERVER_IP"
    exit 0
fi

# Get OS-specific config directory
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    CONFIG_DIR="$HOME/.config/casaverde_server"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    CONFIG_DIR="$HOME/Library/Application Support/casaverde_server"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    CONFIG_DIR="$APPDATA/casaverde_server"
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

# Dynamically get server IP
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if command -v ip >/dev/null; then
        SERVER_IP=$(ip addr show | grep -o "inet [0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}" | grep -v "inet 127\." | head -n 1 | cut -d' ' -f2)
    elif command -v ifconfig >/dev/null; then
        SERVER_IP=$(ifconfig | grep -o "inet [0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}" | grep -v "inet 127\." | head -n 1 | cut -d' ' -f2)
    else
        SERVER_IP=""
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if command -v ifconfig >/dev/null; then
        SERVER_IP=$(ifconfig | grep -o "inet [0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}" | grep -v "inet 127\." | head -n 1 | cut -d' ' -f2)
    else
        SERVER_IP=""
    fi
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    if command -v ipconfig >/dev/null; then
        SERVER_IP=$(ipconfig | grep -o "IPv4 Address.*[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}" | grep -v "127\." | head -n 1 | grep -o "[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}")
    else
        SERVER_IP=""
    fi
else
    SERVER_IP=""
fi

# Default to 10.0.0.12:3003 if no IP found or use environment variable
SERVER_IP="${SERVER_IP:-10.0.0.12}:3003"

# Create config directory
mkdir -p "$CONFIG_DIR"
cd "$CONFIG_DIR"

# Generate certificates
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes -subj "/CN=$SERVER_IP"

# Set permissions
chmod 600 server.key
chmod 644 server.crt

# Open port 3003
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if command -v ufw >/dev/null; then
        echo "Opening port 3003 with ufw..."
        sudo ufw allow 3003/tcp
        sudo ufw reload
    else
        echo "ufw not found. Please manually open port 3003:"
        echo "  sudo firewall-cmd --add-port=3003/tcp --permanent"
        echo "  sudo firewall-cmd --reload"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Please manually open port 3003 on macOS:"
    echo "  sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add-port 3003"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    echo "Please manually open port 3003 on Windows:"
    echo "  netsh advfirewall firewall add rule name=\"casaverde_server\" dir=in action=allow protocol=TCP localport=3003"
fi

echo "Certificates generated in $CONFIG_DIR for IP $SERVER_IP"
echo "Run the server with: SERVER_IP=$SERVER_IP:3003 cargo run"
