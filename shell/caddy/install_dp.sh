#!/bin/bash

# 依赖库安装

ROOT_DIR=/www/file-server


# lib start
LIB_DIR=$ROOT_DIR/lib

if [ ! -d $LIB_DIR ];then
	mkdir -p $LIB_DIR
fi


if [ ! -f ${LIB_DIR}/rpcsvc-proto-1.4.tar.gz ];then
	wget -O ${LIB_DIR}/rpcsvc-proto-1.4.tar.gz https://github.com/thkukuk/rpcsvc-proto/releases/download/v1.4/rpcsvc-proto-1.4.tar.gz
fi


# lib end



# soft start
SOFT_DIR=$ROOT_DIR/soft

if [ ! -d $SOFT_DIR ];then
	mkdir -p $SOFT_DIR
fi

# - mysql start
MY_DIR=$SOFT_DIR/mysql
if [ ! -d $MY_DIR ];then
	mkdir -p $MY_DIR
fi
if [ ! -f ${MY_DIR}/mysql-5.5.62.tar.gz ];then
	wget -O ${MY_DIR}/mysql-5.5.62.tar.gz https://dev.mysql.com/get/Downloads/MySQL-5.5/mysql-5.5.62.tar.gz
fi

if [ ! -f ${MY_DIR}/mysql-5.6.50.tar.gz ];then
	wget -O ${MY_DIR}/mysql-5.6.50.tar.gz https://cdn.mysql.com/Downloads/MySQL-5.6/mysql-5.6.50.tar.gz
fi

if [ ! -f ${MY_DIR}/mysql-boost-5.7.39.tar.gz ];then
	wget -O ${MY_DIR}/mysql-boost-5.7.39.tar.gz https://cdn.mysql.com/archives/mysql-5.7/mysql-boost-5.7.39.tar.gz
fi

if [ ! -f ${MY_DIR}/mysql-boost-8.0.30.tar.gz ];then
	wget -O ${MY_DIR}/mysql-boost-8.0.30.tar.gz https://cdn.mysql.com/archives/mysql-8.0/mysql-boost-8.0.30.tar.gz
fi


# - mysql end

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






