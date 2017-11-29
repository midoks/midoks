#!/bin/sh

solr=` ps -ef|grep solr|grep -v "grep"| grep -v "solr_check.sh"`
time=`date "+%Y-%m-%d %H:%M:%S"`

if [ "$solr" = "" ];then
    echo "$time" "running fail"
    /sbin/service solr restart
else 
    echo "$time" "running ok"
fi 

