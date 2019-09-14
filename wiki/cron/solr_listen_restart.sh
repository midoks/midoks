#!/bin/bash

#监听solr[挂了立即重启]


for((i=1;i<=30;i++));do


solr_pid=$(ps -ef|grep java | grep solr|grep -v grep | awk '{print $2}' )

if [ "$solr_pid" == "" ]; then
    service solr restart
    echo 'solr挂了,已经重启'
fi

sleep 2
done