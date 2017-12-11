#!/bin/bash
# this script run at 00:00

# The Nginx Logs Path

#生成的日志文件名字
logs_name="pifa360"
#要切割的日志
logs_file="/usr/local/nginx/logs/"
#日志PID
nginx_pid=$logs_file"nginx.pid"
#原生态日志(要切割日志)
logs_file_source=$logs_file$logs_name".log"
#日志路径(不存在,自动生成)
logs_file_path=$logs_file$logs_name"/"

#动态生成的
logs_file_direct=${logs_file_path}$(date -d "yesterday" +"%Y")"/"$(date -d "yesterday" +"%m")"/"

#生成的相应日志文件
mkdir -p $logs_file_direct
#把日志文件转移到对应的年月目录中
mv $logs_file_source $logs_file_direct$logs_name"_"$(date -d "yesterday" +"%Y%m%d")".log"

#平滑重启
kill -USR1 `cat $nginx_pid`
#echo `cat $nginx_pid`

#删除三月前的日志
#三月前的年份
NYEAR=$(date -d "-3 month" +"%Y")
#三月前的月份
NMONTH=$(date -d "-3 month" +"%m")

#logs_dir="pifa360"
#要删除的日志的基础目录
#logs_file="/usr/local/nginx/logs/$logs_dir/"

#目录
need_delete_dir=$logs_file_path$NYEAR"/"$NMONTH"/"

#测试
#mkdir -p $need_delete_dir

#删除
if [ -d $need_delete_dir ];then
	/bin/rm -rf $need_delete_dir
fi
#设置crontab, 每天凌晨00:00切割nginx访问日志
#crontab -e
# OO OO * * * /bin/bash /usr/local/nginx/sbin/nginx_log.sh
