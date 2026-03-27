#!/bin/bash
set -e

echo "Post-removal cleanup for Network Probe..."

# Ask if user wants to remove configuration files
if [ -d /etc/network-probe ]; then
    echo "Configuration directory /etc/network-probe still exists."
    echo "You may want to remove it manually if you don't need the configuration anymore."
    echo "  sudo rm -rf /etc/network-probe"
fi

if [ -d /var/log/network-probe ]; then
    echo "Log directory /var/log/network-probe still exists."
    echo "You may want to remove it manually if you don't need the logs anymore."
    echo "  sudo rm -rf /var/log/network-probe"
fi

echo "Network Probe removed successfully!"
