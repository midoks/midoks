#!/bin/bash

ps axf -o "pid %cpu command" | awk '{if($2>=75.0) print $1}' | while read procid
do
kill -9 $procid
done
