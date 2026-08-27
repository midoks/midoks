#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/master.sh | bash


tee /etc/systemd/system/seaweedfs-master.service << 'EOF'
[Unit]
Description=SeaweedFS Master
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/seaweedfs
ExecStart=/usr/local/bin/weed master \
    -ip=0.0.0.0 \
    -port=9333 \
    -mdir=/opt/seaweedfs/data/master \
    -defaultReplication=000
Restart=always
RestartSec=5
LimitNOFILE=65536
StandardOutput=append:/var/log/seaweedfs/master.log
StandardError=append:/var/log/seaweedfs/master.log

[Install]
WantedBy=multi-user.target
EOF


systemctl daemon-reload
systemctl enable seaweedfs-master
systemctl start seaweedfs-master