#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/check_gfw.sh | bash

run_time=`date "+%Y-%m-%d %H:%M:%S"`
find_gf=`ps -ef|grep 'xyjump/site' | grep -v grep`
echo ${find_gf}
if [ "${find_gf}" != ""  ];then
    echo "site running"
    echo "${run_time} site running" > /tmp/check_gfw.log
else
	sh /etc/init.d/xyjump
	echo "site restart"
	echo "${run_time} site restart" > /tmp/check_gfw_restart.log
fi

ps -ef|grep xyjump
cat /usr/local/xyjump/port.txt
cat /tmp/check_gfw.log
cat /tmp/check_gfw_restart.log


find_node=`ps -ef|grep 'cloud-node' | grep -v grep`
echo ${find_node}

find_node=`ps -ef|grep 'cloud-node' | grep -v grep`
if [ "${find_gf}" != ""  ];then
    echo "cloud-node running"
else
	cloud-node restart
    echo "cloud-node restart"
fi

