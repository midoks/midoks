#!/bin/bash

# 获取内存数据，检查内存泄露
# */1 * * * * ~/record/prog_mem.sh

# pidstat  -r -u -h -C gop2p |awk 'NR==4{print $12}'

prog_name="imail"
prog_mem=$(pidstat  -r -u -h -C $prog_name |awk 'NR==4{print $12}')
time=$(date "+%Y-%m-%d %H:%M:%S")
echo $time"\tmemory(Byte)\t"$prog_mem >>~/record/prog_mem.log