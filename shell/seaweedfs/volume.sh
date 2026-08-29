#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/volume.sh | bash

# mkdir -p /var/log/seaweedfs
# tail -f /var/log/seaweedfs/volume.log

# ps -ef | grep "weed filer"

rm -rf /data/seaweedfs/data/volume
mkdir -p /data/seaweedfs/data/volume
chmod 755 /data/seaweedfs/data/volume

# telnet 10.210.0.11 19333

# 预留 7% 空间
# 增加文件描述符限制，对高并发场景很重要

tee /etc/systemd/system/seaweedfs-volume.service << 'EOF'
[Unit]
Description=SeaweedFS Volume
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
ExecStart=/usr/local/bin/weed volume \
    -ip=10.210.0.11 \
    -dir=/data/seaweedfs/data/volume \
    -mserver=10.210.0.14:9333,10.210.0.12:9333,10.210.0.13:9333 \
    -port=8080 \
    -index=leveldb \
    -max=0 \
    -dataCenter=DefaultDataCenter \
    -rack=DefaultRack \
    -minFreeSpacePercent=7

Restart=always
RestartSec=5
LimitNOFILE=1000000
StandardOutput=append:/var/log/seaweedfs/volume.log
StandardError=append:/var/log/seaweedfs/volume.log

[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload
systemctl enable seaweedfs-volume
systemctl restart seaweedfs-volume
systemctl status seaweedfs-volume


# curl http://127.0.0.1:9333/dir/status | jq .

# weed shell -master=127.0.0.1:9333

systemctl daemon-reload
systemctl enable seaweedfs-volume
systemctl start seaweedfs-volume


# echo "volume.grow -collection=m3u8 -count=1" | weed shell
systemctl stop seaweedfs-volume

# journalctl -u seaweedfs-volume -f




/usr/local/bin/weed volume \
    -dir=/data/seaweedfs/data/volume \
    -mserver=10.210.0.11:9333 \
    -port=8080 \
    -index=leveldb \
    -max=0 \
    -minFreeSpacePercent=7