#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/check_gfw.sh | bash
TZ='Asia/Shanghai'
run_time=$(TZ='Asia/Shanghai' date "+%Y-%m-%d %H:%M:%S")
find_gf=`ps -ef|grep 'xyjump/site' | grep -v grep`
echo ${find_gf}
if [ "${find_gf}" != ""  ];then
    echo "site running"
    echo "${run_time} site running" > /tmp/check_gfw.log
else
	pkill site
	sh /etc/init.d/xyjump
	echo "site restart"
	echo "${run_time} site restart" > /tmp/check_gfw_restart.log
fi

ps -ef|grep xyjump
cat /usr/local/xyjump/port.txt
cat /tmp/check_gfw.log

if [ -f /tmp/check_gfw_restart.log ];then
	cat /tmp/check_gfw_restart.log
fi

find_node=`ps -ef|grep 'cloud-node' | grep -v grep`
echo ${find_node}
if [ "${find_gf}" != ""  ];then
    echo "cloud-node running"
else
	cloud-node restart
    echo "cloud-node restart"
fi

find_nezha=`ps -ef|grep 'nezha-agent' | grep -v grep`
echo ${find_nezha}
if [ "${find_nezha}" != ""  ];then
    echo "nezha-agent running"
else
	service nezha-agent restart
    echo "nezha-agent restart"
fi


run_time_end=$(TZ='Asia/Shanghai' date "+%H:%M")

if [ "$run_time_end" == "01:15" ];then
	cloud-node restart
	pkill site
	sh /etc/init.d/xyjump
fi