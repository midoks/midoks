#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/s3.sh | bash


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
      "actions": ["Read", "Write", "List", "Admin"]
    },
    {
      "name": "app-user",
      "credentials": [
        {
          "accessKey": "admin",
          "secretKey": "I0K4GMMv8mS9"
        }
      ],
      "actions": ["Read", "Write"],
      "buckets": ["m3u8"]
    }
  ]
}
EOF

tee /etc/seaweedfs/s3.env << 'EOF'
# S3 Gateway 配置
# Filer 地址列表（支持 HA 故障转移）
FILER_ADDRS="127.0.0.1:8888"
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
WorkingDirectory=/opt/seaweedfs
EnvironmentFile=/etc/seaweedfs/s3.env
ExecStart=/usr/local/bin/weed s3 \
    -filer=${FILER_ADDRS} \
    -port=${S3_PORT} \
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
systemctl enable seaweedfs-s3
systemctl start seaweedfs-s3

# systemctl restart seaweedfs-s3
# systemctl status seaweedfs-s3
# journalctl -u seaweedfs-s3 -f



# /usr/local/bin/weed s3 \
# -filer=127.0.0.1:8888 \
# -port=8333 \
# -domainName=s3.example.com