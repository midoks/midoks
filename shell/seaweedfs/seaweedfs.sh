#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/seaweedfs.sh | bash

# --- 函数：获取最新版本号 ---
get_latest_version() {
    curl -s https://api.github.com/repos/seaweedfs/seaweedfs/releases/latest | \
    grep '"tag_name"' | \
    sed -E 's/.*"([^"]+)".*/\1/'
}


# --- 1. 下载 SeaweedFS (与单机版相同) ---
echo ">>> 1. 开始下载 SeaweedFS..."
if [ "$VERSION" == "latest" ]; then
    VERSION=$(get_latest_version)
    echo "    获取到最新版本: $VERSION"
fi

# --- 配置变量 ---
INSTALL_DIR="/usr/local/bin"
DATA_DIR="/data/seaweedfs"
MASTER_IP="<MASTER_IP>:9333"  # 替换为你的 Master 节点 IP
THIS_IP=$(hostname -I | awk '{print $1}')
VERSION="latest"


cd /tmp

TARBALL="linux_amd64_large_disk.tar.gz"
DOWNLOAD_URL="https://github.com/seaweedfs/seaweedfs/releases/download/${VERSION}/freebsd_amd64_large_disk.tar.gz"
if [ ! -f linux_amd64_large_disk.tar.gz ];then
	wget -O linux_amd64_large_disk.tar.gz $DOWNLOAD_URL
fi

# weed master -ip=192.168.1.10 -port=9333 -mdir=/data/seaweedfs/master -peers=192.168.1.10:9333,192.168.1.11:9333,192.168.1.12:9333

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

# --- 3. 根据角色启动服务 ---
echo ">>> 3. 以 '$NODE_ROLE' 角色启动服务..."

sudo mkdir -p "$DATA_DIR"

case $NODE_ROLE in
    master)
        echo "    启动 Master 节点..."
        nohup weed master -ip="$THIS_IP" -port=9333 -mdir="$DATA_DIR/master" \
            > /tmp/seaweedfs-master.log 2>&1 &
        ;;

    volume)
        echo "    启动 Volume 节点..."
        nohup weed volume -ip="$THIS_IP" -port=8080 \
            -dir="$DATA_DIR/volume" \
            -master="$MASTER_IP" \
            -max=0 \
            > /tmp/seaweedfs-volume.log 2>&1 &
        ;;

    *)
        echo "错误: 不支持的 NODE_ROLE: $NODE_ROLE"
        exit 1
        ;;
esac

echo ">>> 节点部署完成！"
echo "    角色: $NODE_ROLE"
echo "    日志文件: /tmp/seaweedfs-${NODE_ROLE}.log"