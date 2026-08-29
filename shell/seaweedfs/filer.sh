#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/filer.sh | bash


# tail -f /var/log/seaweedfs/filer.log
rm -rf /opt/seaweedfs/filerldb2

mkdir -p /opt/seaweedfs/filer
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
    -ip=10.210.0.11 \
    -master=10.210.0.14:9333,10.210.0.12:9333,10.210.0.13:9333 \
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
systemctl restart seaweedfs-filer
systemctl status seaweedfs-filer

systemctl daemon-reload
systemctl enable seaweedfs-filer
systemctl start seaweedfs-filer
# journalctl -u seaweedfs-filer -f


# curl http://127.0.0.1:9333/vol/status | jq .