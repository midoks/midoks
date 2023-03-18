#!/bin/bash


CADDY_FILE=https://github.com/caddyserver/caddy/releases/download/v2.6.4/caddy_2.6.4_linux_amd64.tar.gz
TMP_FILE=/tmp/caddy_2.6.4_linux_amd64.tar.gz
if [ ! -f $TMP_FILE ];then
	echo "wget --no-check-certificate -O $TMP_FILE $CADDY_FILE"
	wget --no-check-certificate -O $TMP_FILE $CADDY_FILE
fi

cd /tmp
tar -zxvf caddy_2.6.4_linux_amd64.tar.gz


# /usr/lib/systemd/system
# /lib/systemd/system


echo "111"
