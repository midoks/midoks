#!/bin/bash

# 依赖库安装

get_latest_release() {
    curl -sL "https://api.github.com/repos/$1/releases/latest" | grep '"tag_name":' | cut -d'"' -f4
}

ROOT_DIR=/www/file-server


# lib start
LIB_DIR=$ROOT_DIR/lib

if [ ! -d $LIB_DIR ];then
	mkdir -p $LIB_DIR
fi


if [ ! -f ${LIB_DIR}/rpcsvc-proto-1.4.tar.gz ];then
	wget -O ${LIB_DIR}/rpcsvc-proto-1.4.tar.gz https://github.com/thkukuk/rpcsvc-proto/releases/download/v1.4/rpcsvc-proto-1.4.tar.gz
fi

if [ ! -f ${LIB_DIR}/freetype-2.12.1.tar.gz ];then
	wget -O ${LIB_DIR}/freetype-2.12.1.tar.gz https://download.savannah.gnu.org/releases/freetype/freetype-2.12.1.tar.gz
fi

if [ ! -f ${LIB_DIR}/freetype-2.7.1.tar.gz ];then
	wget -O ${LIB_DIR}/freetype-2.7.1.tar.gz https://download.savannah.gnu.org/releases/freetype/freetype-2.7.1.tar.gz
fi

if [ ! -f ${LIB_DIR}/icu4c-52_2-src.tgz ];then
	wget -O ${LIB_DIR}/icu4c-52_2-src.tgz https://github.com/unicode-org/icu/releases/download/release-52-2/icu4c-52_2-src.tgz
fi

if [ ! -f ${LIB_DIR}/icu4c-55_2-src.tgz ];then
	wget -O ${LIB_DIR}/icu4c-55_2-src.tgz https://github.com/unicode-org/icu/releases/download/release-55-2/icu4c-55_2-src.tgz
fi


if [ ! -f ${LIB_DIR}/libiconv-1.15.tar.gz ];then
	wget -O ${LIB_DIR}/libiconv-1.15.tar.gz https://github.com/midoks/mdserver-web/releases/download/init/libiconv-1.15.tar.gz
fi

if [ ! -f ${LIB_DIR}/libmcrypt-2.5.8.tar.gz ];then
	wget -O ${LIB_DIR}/libmcrypt-2.5.8.tar.gz https://sourceforge.net/projects/mcrypt/files/Libmcrypt/2.5.8/libmcrypt-2.5.8.tar.gz
fi

if [ ! -f ${LIB_DIR}/libmemcached-1.0.18.tar.gz ];then
	wget -O ${LIB_DIR}/libmemcached-1.0.18.tar.gz https://launchpad.net/libmemcached/1.0/1.0.18/+download/libmemcached-1.0.18.tar.gz
fi

if [ ! -f ${LIB_DIR}/libsodium-1.0.18-stable.tar.gz ];then
	wget -O ${LIB_DIR}/libsodium-1.0.18-stable.tar.gz https://download.libsodium.org/libsodium/releases/libsodium-1.0.18-stable.tar.gz
fi

if [ ! -f ${LIB_DIR}/libzip-1.3.2.tar.gz ];then
	wget -O ${LIB_DIR}/libzip-1.3.2.tar.gz https://nih.at/libzip/libzip-1.3.2.tar.gz
fi

if [ ! -f ${LIB_DIR}/oniguruma-6.9.4.tar.gz ];then
	wget -O ${LIB_DIR}/oniguruma-6.9.4.tar.gz https://github.com/kkos/oniguruma/archive/v6.9.4.tar.gz
fi


if [ ! -f ${LIB_DIR}/openssl-1.1.1p.tar.gz ];then
	wget -O ${LIB_DIR}/openssl-1.1.1p.tar.gz https://www.openssl.org/source/openssl-1.1.1p.tar.gz
fi


if [ ! -f ${LIB_DIR}/openssl-1.0.2q.tar.gz ];then
	wget -O ${LIB_DIR}/openssl-1.0.2q.tar.gz https://github.com/midoks/mdserver-web/releases/download/init/openssl-1.0.2q.tar.gz
fi

if [ ! -f ${LIB_DIR}/zlib-1.2.11.tar.gz ];then
	wget -O ${LIB_DIR}/zlib-1.2.11.tar.gz https://github.com/madler/zlib/archive/v1.2.11.tar.gz
fi


# lib end



# soft start
SOFT_DIR=$ROOT_DIR/soft

if [ ! -d $SOFT_DIR ];then
	mkdir -p $SOFT_DIR
fi

# - ftp start
FTP_DIR=$SOFT_DIR/ftp
if [ ! -d $FTP_DIR ];then
	mkdir -p $FTP_DIR
fi

if [ ! -f ${FTP_DIR}/pure-ftpd-1.0.49.tar.gz ];then
	wget -O ${FTP_DIR}/pure-ftpd-1.0.49.tar.gz https://github.com/jedisct1/pure-ftpd/releases/download/1.0.49/pure-ftpd-1.0.49.tar.gz
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

if [ ! -f ${OP_DIR}/openresty-1.21.4.2.tar.gz ];then
	wget -O ${OP_DIR}/openresty-1.21.4.2.tar.gz https://openresty.org/download/openresty-1.21.4.2.tar.gz
fi

# - openresty end



# - webstats start
WB_DIR=$SOFT_DIR/webstats

if [ ! -d $WB_DIR ];then
	mkdir -p $WB_DIR
fi

GEO_VERSION=$(get_latest_release "P3TERX/GeoLite.mmdb")
if [ ! -f ${WB_DIR}/GeoLite2-City.mmdb ];then
	wget --no-check-certificate -O ${WB_DIR}/GeoLite2-City.mmdb https://github.com/P3TERX/GeoLite.mmdb/releases/download/${GEO_VERSION}/GeoLite2-City.mmdb
fi

if [ ! -f ${WB_DIR}/lsqlite3_fsl09y.zip ];then
	wget --no-check-certificate -O ${WB_DIR}/lsqlite3_fsl09y.zip http://lua.sqlite.org/index.cgi/zip/lsqlite3_fsl09y.zip?uuid=fsl_9y
fi

if [ ! -f ${WB_DIR}/luarocks-3.5.0.tar.gz ];then
	wget --no-check-certificate -O ${WB_DIR}/luarocks-3.5.0.tar.gz http://luarocks.org/releases/luarocks-3.5.0.tar.gz
fi

# - webstats end

# soft end






