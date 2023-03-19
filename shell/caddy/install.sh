#!/bin/bash

CADDY_FILE=https://github.com/caddyserver/caddy/releases/download/v2.6.4/caddy_2.6.4_linux_amd64.tar.gz
TMP_FILE=/tmp/caddy_2.6.4_linux_amd64.tar.gz
if [ ! -f $TMP_FILE ];then
	echo "wget --no-check-certificate -O $TMP_FILE $CADDY_FILE"
	wget --no-check-certificate -O $TMP_FILE $CADDY_FILE
fi

mkdir -p /opt/caddy

if [ ! -f /opt/caddy/caddy ];then

	cd /tmp
	tar -zxvf caddy_2.6.4_linux_amd64.tar.gz
	cp -rf /tmp/caddy /opt/caddy/caddy
fi

systemd_dir=/usr/lib/systemd/system
if [ ! -d /usr/lib/systemd/system ];then
	systemd_dir=/lib/systemd/system
fi

# if [ ! -f $systemd_dir/caddy.service ];then
rm -rf $systemd_dir/caddy.service
wget -O $systemd_dir/caddy.service https://raw.githubusercontent.com/midoks/midoks/master/shell/caddy/caddy.service
# fi

rm -rf /tmp/LICENSE
rm -rf /tmp/README.md
rm -rf /tmp/caddy_2.6.4_linux_amd64.tar.gz

echo "Installation succeeded"
