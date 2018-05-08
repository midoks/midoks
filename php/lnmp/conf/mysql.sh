#! /bin/bash

mysql_port=3306
mysql_username="root"
mysql_password="test159753"

#启动mysql
function_start_mysql()
{
	printf "strart mysql...\n"
	/usr/local/mysql/bin/mysqld_safe --user=mysql&
}

#停止mysql
function_stop_mysql()
{
	printf "stop mysql ..\n"
	/usr/local/mysql/bin/mysqladmin -u${mysql_username} -p${mysql_password} shutdown
}

#重新启动mysql
function_restart_mysql()
{
	printf "restarting mysql ...\n"
	function_stop_mysql
	sleep 5 #暂停5s
	function_start_mysql
}

#kill mysql进程
function_kill_mysql()
{
	kill -9 $(ps -ef | grep 'bin/mysql_safe' | awk '{printf $2}')
	kill -9 $(ps -ef | grep 'libexec/mysqld'  | awk '{printf $2}')
}

###根据参数来控制MySQL
if [ "$1" = "start"];then
	function_start_mysql
elif [ "$1" = "stop" ];then
	function_restart_mysql
elif [ "$1" = "restart"];then
	function_restart_mysql
elif [ "$1" = "kill"];then
	function_kill_mysql
else
	printf "all fail"
fi
