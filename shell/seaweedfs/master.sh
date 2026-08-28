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
    -ip=154.12.53.22 \
    -port=9333 \
    -volumeSizeLimitMB=2048000 \ # 例如 2TB（2048 * 1024 MB）
    -mdir=/opt/seaweedfs/data/master \
    -defaultReplication=000 
Restart=always
RestartSec=5
LimitNOFILE=1000000
StandardOutput=append:/var/log/seaweedfs/master.log
StandardError=append:/var/log/seaweedfs/master.log

[Install]
WantedBy=multi-user.target
EOF


systemctl daemon-reload
systemctl enable seaweedfs-master
systemctl start seaweedfs-master

systemctl daemon-reload
systemctl restart seaweedfs-master
systemctl status seaweedfs-master
# journalctl -u seaweedfs-master -f