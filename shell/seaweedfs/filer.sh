#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/filer.sh | bash


tee /etc/systemd/system/seaweedfs-filer.service << 'EOF'
[Unit]
Description=SeaweedFS Filer
After=network.target seaweedfs-filer.service
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/seaweedfs
ExecStart=/usr/local/bin/weed filer \
    -master=127.0.0.1:9333 \
    -port=8888
Restart=always
RestartSec=5
LimitNOFILE=65536
StandardOutput=append:/var/log/seaweedfs/filer.log
StandardError=append:/var/log/seaweedfs/filer.log

[Install]
WantedBy=multi-user.target
EOF


systemctl daemon-reload
systemctl enable seaweedfs-filer
systemctl start seaweedfs-filer

# systemctl restart seaweedfs-filer

# systemctl status seaweedfs-filer
# journalctl -u seaweedfs-filer -f

