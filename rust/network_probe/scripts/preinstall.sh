#!/bin/bash
set -e

echo "Pre-installation checks for Network Probe..."

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

# Check if previous installation exists
if [ -f /etc/network-probe/config.yaml ]; then
    echo "Backing up existing configuration..."
    cp /etc/network-probe/config.yaml /etc/network-probe/config.yaml.backup
fi

echo "Pre-installation checks completed successfully."
