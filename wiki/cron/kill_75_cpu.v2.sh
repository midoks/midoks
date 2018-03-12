#!/bin/bash


/bin/ps axf -o "pid %cpu command"  | while read LINE
do
	echo $LINE | awk '{if($2>=75.0) print $1 }' | while read pid
	do
		kill -9 $pid
		echo $LINE,killed
	done

done

