#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/master.sh | bash

# mkdir -p /var/log/seaweedfs
# tail -f /var/log/seaweedfs/master.log
# tail -f /var/log/seaweedfs/volume.log


mkdir -p  /data/seaweedfs/data/master
tee /etc/systemd/system/seaweedfs-master.service << 'EOF'
[Unit]
Description=SeaweedFS Master
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
ExecStart=/usr/local/bin/weed \
    -logdir=/var/log/seaweedfs \
    -log_max_size_mb=100 \
    -log_max_files=3 \
    -v=4 \
    master \
    -ip=10.210.0.14 \
    -port=9333 \
    -peers=10.210.0.14:9333,10.210.0.12:9333,10.210.0.13:9333 \
    -volumeSizeLimitMB=30720 \
    -mdir=/data/seaweedfs/data/master \
    -defaultReplication="001" \
    -volumePreallocate 
Restart=always
RestartSec=5
LimitNOFILE=1000000
StandardOutput=append:/var/log/seaweedfs/master.log
StandardError=append:/var/log/seaweedfs/master.log

[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload
systemctl restart seaweedfs-master
systemctl status seaweedfs-master


systemctl daemon-reload
systemctl enable seaweedfs-master
systemctl restart seaweedfs-master
systemctl stop seaweedfs-master
# journalctl -u seaweedfs-master -f

# -volumeSizeLimitMB=2048000 例如 2TB（2048 * 1024 MB）

# /usr/local/bin/weed master \
#     -ip=10.210.0.12 \
#     -port=9333 \
#     -peers=10.210.0.11:9333,10.210.0.13:9333,10.210.0.12:9333 \
#     -volumeSizeLimitMB=2048000 \
#     -mdir=/data/seaweedfs/data/master \
#     -defaultReplication="001"


/usr/local/bin/weed master \
    -ip=10.210.0.12 \
    -port=9333 \
    -peers=10.210.0.11:9333,10.210.0.13:9333,10.210.0.12:9333 \
    -volumeSizeLimitMB=2048000 \
    -mdir=/data/seaweedfs/data/master \
    -logdir=/var/log/seaweedfs \
    -log_max_size_mb=100 \
    -log_max_files=3 \
    -defaultReplication="001"



