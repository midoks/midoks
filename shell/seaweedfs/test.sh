#!/bin/bash

# apt install -y rclone

curl -F "file=@/Users/midoks/Desktop/a.ping" http://154.12.53.22:8888/test/

curl -F "file=@/tmp/test.txt" "http://127.0.0.1:8888/test/"

curl -F "file=@/tmp/test.txt" "http://127.0.0.1:8888/xxx/"

# rclone sync source:m3u8 dest:m3u8 --progress
# apt install mc -y

curl -L -O https://dl.min.io/client/mc/release/linux-amd64/mc
chmod +x mc


# ./mc alias set src http://38.246.114.74:9090/buckets/m3u8 admin I0K4GMMv8mS9

# ./mc alias set dst http://154.12.53.22:8333 app I0K4GMMv8mS9