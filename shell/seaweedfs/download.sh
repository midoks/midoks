#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/download.sh | bash

# rm -rf /usr/local/bin/weed

# --- 函数：获取最新版本号 ---
get_latest_version() {
    curl -s https://api.github.com/repos/seaweedfs/seaweedfs/releases/latest | \
    grep '"tag_name"' | \
    sed -E 's/.*"([^"]+)".*/\1/'
}

# --- 配置变量 ---
INSTALL_DIR="/usr/local/bin"
DATA_DIR="/data/seaweedfs"
MASTER_IP="<MASTER_IP>:9333"  # 替换为你的 Master 节点 IP
THIS_IP=$(hostname -I | awk '{print $1}')
VERSION="latest"
NODE_ROLE="master"

# --- 1. 下载 SeaweedFS (与单机版相同) ---
echo ">>> 1. 开始下载 SeaweedFS..."
if [ "$VERSION" == "latest" ]; then
    VERSION=$(get_latest_version)
    echo "    获取到最新版本: $VERSION"
fi

cd /tmp

TARBALL="linux_amd64_full_large_disk.tar.gz"
DOWNLOAD_URL="https://github.com/seaweedfs/seaweedfs/releases/download/${VERSION}/linux_amd64_full_large_disk.tar.gz"
if [ ! -f linux_amd64_full_large_disk.tar.gz ];then
	wget -O linux_amd64_full_large_disk.tar.gz $DOWNLOAD_URL
fi

wget -q --show-progress "$DOWNLOAD_URL" -O "$TARBALL"
sudo tar -xzf "$TARBALL" -C "$INSTALL_DIR" weed
sudo chmod +x "$INSTALL_DIR/weed"
rm -f "$TARBALL"


# --- 参数检查 ---
if [ -z "$NODE_ROLE" ]; then
    echo "错误: 请设置 NODE_ROLE 环境变量 (master/volume)"
    exit 1
fi

echo ">>> 2. 验证安装..."
weed version
echo ">>> 节点部署完成！"