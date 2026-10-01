#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/s3.sh | bash


# tail -f /var/log/seaweedfs/s3.log

mkdir -p /etc/seaweedfs
tee /etc/seaweedfs/s3config.json << 'EOF'
{
  "identities": [
    {
      "name": "admin",
      "credentials": [
        {
          "accessKey": "admin",
          "secretKey": "I0K4GMMv8mS9"
        }
      ],
      "actions": ["Read", "Write", "List", "Admin"],
      "buckets": ["m3u8"]
    },
    {
      "name": "app",
      "credentials": [
        {
          "accessKey": "app",
          "secretKey": "I0K4GMMv8mS9"
        }
      ],
      "actions": ["Read", "Write"],
      "buckets": ["m3u8"]
    },
    {
      "name": "anonymous",
      "actions": ["Read"],
      "buckets": ["m3u8"]
    }
  ]
}
EOF

tee /etc/seaweedfs/s3.env << 'EOF'
# S3 Gateway 配置
# Filer 地址列表（支持 HA 故障转移）
FILER_ADDRS="10.210.0.11:8888,10.210.0.12:8888,10.210.0.13:8888,10.210.0.14:8888,10.210.0.15:8888"
S3_PORT=8333
# 可选，S3 服务域名
DOMAIN_NAME=s3.example.com
# 可选，IAM 配置文件
CONFIG_FILE="/etc/seaweedfs/s3config.json"
EOF
chmod 600 /etc/seaweedfs/s3.env


tee /etc/systemd/system/seaweedfs-s3.service << 'EOF'
[Unit]
Description=SeaweedFS S3 Gateway
After=network.target seaweedfs-s3.service
Wants=network.target

[Service]
Type=simple
User=root
Group=root
EnvironmentFile=/etc/seaweedfs/s3.env
ExecStart=/usr/local/bin/weed s3 \
    -filer=${FILER_ADDRS} \
    -port=${S3_PORT} \
    -cacheCapacityMB=98304 \
    -idleTimeout=3600 \
    -config=${CONFIG_FILE}
Restart=always
RestartSec=5
LimitNOFILE=65536
StandardOutput=append:/var/log/seaweedfs/s3.log
StandardError=append:/var/log/seaweedfs/s3.log

[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload
systemctl restart seaweedfs-s3
systemctl status seaweedfs-s3

systemctl daemon-reload
systemctl enable seaweedfs-s3
systemctl start seaweedfs-s3

systemctl stop seaweedfs-s3

systemctl daemon-reload
systemctl restart seaweedfs-s3
systemctl status seaweedfs-s3
# journalctl -u seaweedfs-s3 -f


/usr/local/bin/weed s3 \
-filer="10.210.0.11:8888,10.210.0.12:8888,10.210.0.13:8888,10.210.0.14:8888,10.210.0.15:8888" \
-port=8333 \
-config=/etc/seaweedfs/s3config.json 