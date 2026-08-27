#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/volume.sh | bash

mkdir -p /opt/seaweedfs/data/volume
chmod 755 /opt/seaweedfs/data/volume

tee /etc/systemd/system/seaweedfs-volume.service << 'EOF'
[Unit]
Description=SeaweedFS Volume
After=network.target seaweedfs-volume.service
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/seaweedfs
ExecStart=/usr/local/bin/weed volume \
    -dir=/opt/seaweedfs/data/volume \
    -mserver=127.0.0.1:9333 \
    -port=8080 \
    -max=0
Restart=always
RestartSec=5
LimitNOFILE=65536
StandardOutput=append:/var/log/seaweedfs/volume.log
StandardError=append:/var/log/seaweedfs/volume.log

[Install]
WantedBy=multi-user.target
EOF


# curl http://127.0.0.1:9333/dir/status | jq .

# weed shell -master=127.0.0.1:9333

systemctl daemon-reload
systemctl enable seaweedfs-volume
systemctl start seaweedfs-volume


# echo "volume.grow -collection=m3u8 -count=1" | weed shell
# systemctl stop seaweedfs-volume
# systemctl daemon-reload
# systemctl restart seaweedfs-volume
# systemctl status seaweedfs-volume
# journalctl -u seaweedfs-volume -f