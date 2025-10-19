#!/bin/bash

# cmd
# curl -fsSL https://raw.githubusercontent.com/midoks/midoks/refs/heads/master/shell/check_gfw_one.sh | bash
run_time_end=$(TZ='Asia/Shanghai' date "+%M")

# if [ "$run_time_end" == "30" ];then
echo "restart all start ${run_time_end}"
# cloud-node restart
pkill site
sleep 2
sh /etc/init.d/xyjump

echo "${run_time} auto restart" > /tmp/check_auto_restart.log
echo "open sleep 02" >> /tmp/check_auto_restart.log
echo "restart all end"
# fi

if [ -f /tmp/check_auto_restart.log ];then
	cat /tmp/check_auto_restart.log
fi


