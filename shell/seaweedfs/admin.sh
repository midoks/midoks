#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/admin.sh | bash

# rm -rf /usr/local/bin/weed

mkdir -p /etc/seaweedfs
mkdir -p /var/log/seaweedfs

tee /etc/seaweedfs/admin.env << 'EOF'
ADMIN_PORT=23646
MASTER_ADDRS="localhost:9333"
DATA_DIR="/var/lib/seaweedfs-admin"
ADMIN_USER="admin"
ADMIN_PASSWORD="I0K4GMMv8mS9"
URL_PREFIX="/weed99"
EOF
chmod 600 /etc/seaweedfs/admin.env


# 如果使用专用用户，修改为：
# User=seaweedfs
# Group=seaweedfs
# 日志管理：输出到 systemd journal 和文件（可选）
# 如果不想保留文件，使用 StandardOutput=journal 即可

tee /etc/systemd/system/seaweedfs-admin.service << 'EOF'
[Unit]
Description=SeaweedFS Admin UI
After=network.target
Wants=network.target
# 如果你希望 Admin UI 在 Master 启动后再启动，可以添加：
# After=seaweedfs-master.service
# Requires=seaweedfs-master.service

[Service]
Type=simple
User=root
Group=root

EnvironmentFile=/etc/seaweedfs/admin.env
ExecStart=/usr/local/bin/weed admin \
    -port=${ADMIN_PORT} \
    -masters=${MASTER_ADDRS} \
    -dataDir=${DATA_DIR} \
    -adminUser=${ADMIN_USER} \
    -adminPassword=${ADMIN_PASSWORD} \
    ${URL_PREFIX:+-urlPrefix=${URL_PREFIX}}

StandardOutput=append:/var/log/seaweedfs/admin.log
StandardError=append:/var/log/seaweedfs/admin.log
Restart=always
RestartSec=10
LimitNOFILE=65536
[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload
systemctl enable seaweedfs-admin
systemctl restart seaweedfs-admin
systemctl status seaweedfs-admin

systemctl enable seaweedfs-admin
systemctl start seaweedfs-admin


# systemctl restart seaweedfs-admin
# systemctl status seaweedfs-admin
# journalctl -u seaweedfs-admin -f

# weed admin \
#   -port=23646 \
#   -masters="localhost:9333" \
#   -dataDir="/var/lib/seaweedfs-admin" \
#   -adminUser=admin \
#   -adminPassword=I0K4GMMv8mS9 \
#   -urlPrefix=weed99

