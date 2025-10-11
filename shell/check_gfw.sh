#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/check_gfw.sh | bash

find_gf=`ps -ef|grep 'xyjump/site' | grep -v grep`
echo ${find_gf}
if [ "${find_gf}" != ""  ];then
    echo "site running"
    echo "site running" > /tmp/check_gfw.log
else
	sh /etc/init.d/xyjump
	echo "site restart"
	echo "site restart" > /tmp/check_gfw.log
fi

ps -ef|grep xyjump
cat /usr/local/xyjump/port.txt

