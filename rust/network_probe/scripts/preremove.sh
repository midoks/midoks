#!/bin/bash
set -e

echo "Pre-removal cleanup for Network Probe..."

# Stop the service if it's running
if command -v systemctl &> /dev/null; then
    if systemctl is-active --quiet network-probe; then
        echo "Stopping Network Probe service..."
        systemctl stop network-probe
    fi
    
    if systemctl is-enabled --quiet network-probe; then
        echo "Disabling Network Probe service..."
        systemctl disable network-probe
    fi
fi

echo "Pre-removal cleanup completed."
