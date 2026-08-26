#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/seaweedfs/seaweedfs.sh | bash


cd /tmp

if [ ! -f linux_amd64_large_disk.tar.gz ];then
	wget -O linux_amd64_large_disk.tar.gz https://github.com/seaweedfs/seaweedfs/releases/download/4.44/linux_amd64_large_disk.tar.gz
fi

# weed master -ip=192.168.1.10 -port=9333 -mdir=/data/seaweedfs/master -peers=192.168.1.10:9333,192.168.1.11:9333,192.168.1.12:9333