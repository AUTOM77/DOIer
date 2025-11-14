#!/bin/bash

set -e

SERVICE_NAME="doi"
INSTALL_DIR="/usr/bin"
SYSTEMD_DIR="/etc/systemd/system"
SERVICE_FILE="${SYSTEMD_DIR}/${SERVICE_NAME}.service"
PORT="${1:-8080}"
WORKING_DIR="/var/lib/${SERVICE_NAME}"
GITHUB_REPO="AUTOM77/DOIer"

if [ "$(uname -s)" != "Linux" ]; then
    echo "Error: This script only supports Linux"
    exit 1
fi

echo "Detecting system architecture..."
ARCH=$(uname -m)

case "$ARCH" in
    x86_64)
        TARGET="x86_64-unknown-linux-gnu"
        ;;
    aarch64)
        TARGET="aarch64-unknown-linux-gnu"
        ;;
    *)
        echo "Error: Unsupported architecture: $ARCH"
        echo "Supported: x86_64, aarch64"
        exit 1
        ;;
esac

echo "Detected: $ARCH (target: $TARGET)"

RELEASE_URL="https://api.github.com/repos/${GITHUB_REPO}/releases/latest"
echo "Fetching latest release info..."

BINARY_NAME="${SERVICE_NAME}-${TARGET}"
DOWNLOAD_URL=$(curl -s "$RELEASE_URL" | grep "browser_download_url.*${BINARY_NAME}" | cut -d '"' -f 4 | head -n 1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Error: Could not find binary '${BINARY_NAME}' in latest release"
    exit 1
fi

echo "Downloading from: $DOWNLOAD_URL"
TEMP_FILE="/tmp/${SERVICE_NAME}-download"
curl -L -o "$TEMP_FILE" "$DOWNLOAD_URL"

if [ ! -f "$TEMP_FILE" ]; then
    echo "Error: Download failed"
    exit 1
fi

echo "Installing binary to ${INSTALL_DIR}..."
sudo install -m 755 "$TEMP_FILE" "${INSTALL_DIR}/${SERVICE_NAME}"
rm -f "$TEMP_FILE"

sudo mkdir -p "${WORKING_DIR}"

sudo tee "${SERVICE_FILE}" > /dev/null <<EOF
[Unit]
Description=DOI API Service
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=nobody
Group=nogroup
WorkingDirectory=${WORKING_DIR}
ExecStart=${INSTALL_DIR}/${SERVICE_NAME} --port ${PORT}
Restart=always
RestartSec=10
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=${WORKING_DIR}
StandardOutput=journal
StandardError=journal
SyslogIdentifier=${SERVICE_NAME}

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable "${SERVICE_NAME}.service"
sudo systemctl start "${SERVICE_NAME}.service"

sleep 2

if sudo systemctl is-active --quiet "${SERVICE_NAME}.service"; then
    echo "✓ Installation complete. Service running on port ${PORT}"
else
    echo "Error: Service failed to start. Check: journalctl -u ${SERVICE_NAME}"
    exit 1
fi
