#! /bin/sh

function logstash_pid(){

	logstash_pid=$(ps -ef|grep logstash | sed -n '1,1p' | awk '{print $2}')
	echo $logstash_pid
	kill -9 $logstash_pid
}

logstash_pid