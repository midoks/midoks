#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/volume.sh | bash

# mkdir -p /var/lib/seaweedfs-worker
# tail -f /var/log/seaweedfs/worker.log


# ps -ef | grep "weed filer"

# telnet 10.210.0.11 19333
cd /var/lib/seaweedfs-worker && rm -rf ./erasure_coding

/usr/local/bin/weed worker -h

rm -rf /etc/systemd/system/seaweedfs-worker-ec.service
systemctl daemon-reload

mkdir -p /var/lib/seaweedfs-worker
tee /etc/systemd/system/seaweedfs-worker-ec.service << 'EOF'
[Unit]
Description=SeaweedFS worker
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
ExecStart=/usr/local/bin/weed worker \
    -admin=10.210.0.11:23646 \
    -jobType=ec    \
    -workingDir=/data/worker/tmp \
    -metricsPort=9329

Restart=always
RestartSec=5
LimitNOFILE=1000000
StandardOutput=append:/var/log/seaweedfs/worker-ec.log
StandardError=append:/var/log/seaweedfs/worker-ec.log

[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload
systemctl enable seaweedfs-worker-ec
systemctl restart seaweedfs-worker-ec
systemctl status seaweedfs-worker-ec


# curl http://127.0.0.1:9333/dir/status | jq .

# weed shell -master=127.0.0.1:9333

systemctl daemon-reload
systemctl enable seaweedfs-worker-ec
systemctl start seaweedfs-worker-ec

systemctl restart seaweedfs-worker-ec

systemctl status seaweedfs-worker-ec

systemctl stop seaweedfs-worker-ec

# journalctl -u seaweedfs-worker -f



 nc -zv 10.210.0.11 23646



/usr/local/bin/weed worker \
    -admin=10.210.0.11:23646 \
    -workingDir=/data/worker/tmp \
    --heartbeat=5s \
    -maxExecute=28\
    -jobType=erasure_coding

