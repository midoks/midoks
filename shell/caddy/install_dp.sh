#!/bin/bash

# 依赖库安装

ROOT_DIR=/www/file-server


# lib start
LIB_DIR=$ROOT_DIR/lib

if [ ! -d $LIB_DIR ];then
	mkdir -p $LIB_DIR
fi

# lib end



# soft start
SOFT_DIR=$ROOT_DIR/soft

if [ ! -d $SOFT_DIR ];then
	mkdir -p $SOFT_DIR
fi

# - openresty start
OP_DIR=$SOFT_DIR/openresty
if [ ! -d $OP_DIR ];then
	mkdir -p $OP_DIR
fi
if [ ! -f ${OP_DIR}/openresty-1.17.8.2.tar.gz ];then
	wget -O ${OP_DIR}/openresty-1.17.8.2.tar.gz https://openresty.org/download/openresty-1.17.8.2.tar.gz
fi

if [ ! -f ${OP_DIR}/openresty-1.19.3.1.tar.gz ];then
	wget -O ${OP_DIR}/openresty-1.19.3.1.tar.gz https://openresty.org/download/openresty-1.19.3.1.tar.gz
fi

if [ ! -f ${OP_DIR}/openresty-1.21.4.1.tar.gz ];then
	wget -O ${OP_DIR}/openresty-1.21.4.1.tar.gz https://openresty.org/download/openresty-1.21.4.1.tar.gz
fi


# soft end






