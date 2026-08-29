#!/bin/bash

# apt install -y rclone

# systemctl status minio
# cat /usr/lib/systemd/system/minio.service
# cat /etc/default/minio

apt install wrk -y
apt install screen -y
# screen -S mc_migrate
# screen -r mc_migrate

# wrk -t4 -c5000 -d30s http://38.190.177.102:9000/m3u8/video/m3u8/202306/26/001fac37ee02/000000000.ts
# wrk -t4 -c5000 -d30s http://38.190.177.102:8333/m3u8/a.png
# wrk -t4 -c5000 -d30s http://10.210.0.15:8333/m3u8/a.png



# wrk -t4 -c200 -d30s http://38.190.177.102:8333/m3u8/a.png
# wrk -t4 -c5000 -d30s http://10.210.0.15:8333/m3u8/000000000.ts


wrk -t4 -c200 -d30s http://38.190.177.102:8333/m3u8/uploaded_1000w.txt

# wrk -t4 -c100 -d30s http://38.190.177.105:9000/m3u8/video/m3u8/202306/26/001fac37ee02/000000000.ts
# wrk -t4 -c100 -d30s http://38.190.177.105:8333/m3u8/000000000.ts


wrk -t4 -c300 -d30s http://10.210.0.15:9000/m3u8/video/m3u8/202306/26/001fac37ee02/000000000.ts
wrk -t4 -c2000 -d30s http://10.210.0.12:8333/m3u8/000000000.ts


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


./mc alias set src http://10.210.0.12:9000 admin I0K4GMMv8mS9
./mc alias set dst http://10.210.0.15:8333 admin I0K4GMMv8mS9

./mc alias set src http://10.210.0.11:9000 admin I0K4GMMv8mS9
./mc alias set dst http://10.210.0.11:8333 admin I0K4GMMv8mS9

m3u8/video/m3u8/202306

# ./mc ls dst/
./mc mirror --watch --overwrite src/m3u8/video/m3u8/202306/26 dst/m3u8/video/m3u8/202306/26


nohup ./mc mirror --overwrite src/m3u8/video/m3u8/202306/26 dst/m3u8/video/m3u8/202306/26 > mirror.log 2>&1 &
