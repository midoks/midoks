#! /bin/sh

function stop_kafka(){

	kafka_pid2=$(ps -ef|grep kafka| sed -n '1,1p' | awk '{print $2}')
	echo $kafka_pid2
	kill -9 $kafka_pid2

	kafka_pid=$(ps ax | grep -i 'kafka\.Kafka' | grep java | grep -v grep | awk '{print $1}')
	kill -9 $kafka_pid

}


stop_kafka


rm -rf /usr/local/kafka/zookeeper_log/*