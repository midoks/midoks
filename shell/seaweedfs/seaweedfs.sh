#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/seaweedfs.sh | bash


# cd /tmp

# if [ ! -f linux_amd64_large_disk.tar.gz ];then
# 	wget -O linux_amd64_large_disk.tar.gz https://github.com/seaweedfs/seaweedfs/releases/download/4.44/linux_amd64_large_disk.tar.gz
# fi

# weed master -ip=192.168.1.10 -port=9333 -mdir=/data/seaweedfs/master -peers=192.168.1.10:9333,192.168.1.11:9333,192.168.1.12:9333


#!/bin/bash
# seaweedfs_single.sh - SeaweedFS 单机快速部署脚本

set -e  # 遇到错误立即退出

# --- 配置变量 ---
INSTALL_DIR="/usr/local/bin"
DATA_DIR="/data/seaweedfs"
MASTER_IP=$(hostname -I | awk '{print $1}')  # 自动获取本机IP
VERSION="latest"  # 可指定版本，如 "3.92"

# --- 函数：获取最新版本号 ---
get_latest_version() {
    curl -s https://api.github.com/repos/chrislusf/seaweedfs/releases/latest | \
    grep '"tag_name"' | \
    sed -E 's/.*"([^"]+)".*/\1/'
}

# --- 1. 下载 SeaweedFS ---
echo ">>> 1. 开始下载 SeaweedFS..."
if [ "$VERSION" == "latest" ]; then
    VERSION=$(get_latest_version)
    echo "    获取到最新版本: $VERSION"
fi

DOWNLOAD_URL="https://github.com/chrislusf/seaweedfs/releases/download/${VERSION}/linux_amd64.tar.gz"
TARBALL="linux_amd64.tar.gz"

wget -q --show-progress "$DOWNLOAD_URL" -O "$TARBALL"
echo "    下载完成."

# --- 2. 解压并安装 ---
echo ">>> 2. 解压并安装到 $INSTALL_DIR ..."
sudo tar -xzf "$TARBALL" -C "$INSTALL_DIR" weed
sudo chmod +x "$INSTALL_DIR/weed"
rm -f "$TARBALL"

# --- 3. 验证安装 ---
echo ">>> 3. 验证安装..."
weed version

# --- 4. 创建数据目录 ---
echo ">>> 4. 创建数据目录 $DATA_DIR ..."
sudo mkdir -p "$DATA_DIR"/{master,volume,filer}

# --- 5. 启动服务 (后台运行) ---
echo ">>> 5. 启动 SeaweedFS 服务 (使用 'weed server' 命令)..."
# 'weed server' 命令会同时启动 master, volume, 和 filer[reference:2][reference:3]
nohup weed server -dir="$DATA_DIR" -ip="$MASTER_IP" -filer -s3 > /tmp/seaweedfs.log 2>&1 &

echo ">>> 部署完成！"
echo "    Master 管理界面: http://$MASTER_IP:9333"
echo "    S3 API 端点: http://$MASTER_IP:8333"
echo "    日志文件: /tmp/seaweedfs.log"