#!/bin/bash

# apt install -y rclone

# systemctl status minio
# cat /usr/lib/systemd/system/minio.service
# cat /etc/default/minio

curl http://127.0.0.1:9333/dir/assign  


curl -F "file=@/Users/midoks/Desktop/a.ping" http://154.12.53.22:8888/test/

curl -F "file=@/tmp/test.txt" "http://127.0.0.1:8888/test/"

curl -F "file=@/tmp/test.txt" "http://127.0.0.1:8888/xxx/"

# rclone sync source:m3u8 dest:m3u8 --progress
# apt install mc -y

curl -L -O https://dl.min.io/client/mc/release/linux-amd64/mc
chmod +x mc

export AWS_ACCESS_KEY_ID=WS32CSKSBZRTN5UXYD50
export AWS_SECRET_ACCESS_KEY=BhTky6gXKLgmb3xUQx5OPqKKDPoC97ofpZyF5jimbv

# ./mc alias set dst http://154.12.53.22:8333 admin I0K4GMMv8mS9

# ./mc ls dst/

# ./mc alias set src http://38.246.114.74:9000 admin I0K4GMMv8mS9

# ./mc alias set dst http://154.12.53.22:8333 WS32CSKSBZRTN5UXYD50 BhTky6gXKLgmb3xUQx5OPqKKDPoC97ofpZyF5jimbv


# ./mc ls dst/
./mc mirror --overwrite src/m3u8 dst/m3u8