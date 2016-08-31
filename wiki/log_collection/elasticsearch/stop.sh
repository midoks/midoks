#! /bin/sh

function elastic_pid(){

	elastic_pid=$(ps -ef|grep elastic | sed -n '1,1p' | awk '{print $2}')
	echo $elastic_pid
	kill -9 $elastic_pid
}

elastic_pid
elastic_pid