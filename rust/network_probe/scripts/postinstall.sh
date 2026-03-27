#!/bin/bash
set -e

echo "Post-installation setup for Network Probe..."

# Create config directory if it doesn't exist
mkdir -p /etc/network-probe

# Set proper permissions
chmod 755 /usr/bin/network-probe
chmod -R 755 /etc/network-probe

# Create log directory
mkdir -p /var/log/network-probe
chmod 755 /var/log/network-probe

# Check if systemd is available
if command -v systemctl &> /dev/null; then
    echo "Systemd detected. You can install the service using:"
    echo "  sudo network-probe install"
else
    echo "Systemd not detected. You may need to configure service manually."
fi

echo "Network Probe installed successfully!"
echo "Run 'network-probe --help' to see available commands."
