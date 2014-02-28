#! /bin/sh
echo "start..."

yum -y install gcc gcc-c++ autoconf libjpeg libjpeg-devel libpng libpng-devel freetype freetype-devel libxml2 libxml2-devel zlib zlib-devel \
glibc glibc-devel glib2 glib2-devel bzip2 bzip2-devel ncurses ncurses-devel curl curl-devel e2fsprogs e2fsprogs-devel krb5 krb5-devel libidn\
libidn-devel openssl openssl-devel openldap openldap-devel nss_ldap openldap-clients openldap-servers pcre* db4-devel libXpm-devel \
gmp-devel libc-client-devel unixODBC-devel postgresql-devel sqlite-devel aspell-devel net-snmp-devel libxslt-devel mysql-devel libmcrypt \
libmhash libevent acl
yum -y install gd gd2 gd-devel gd2-devel

ulimit -SHn 65535

cd ./source

/bin/tar zxvf libmcrypt-2.5.8.tar.gz
cd libmcrypt-2.5.8/
./configure
make && make install
cd ../

cd libmcrypt-2.5.8/libltdl
 ./configure --enable-ltdl-install
make && make install
cd ../

ln -s /usr/local/lib/libmcrypt.la /usr/lib/libmcrypt.la
ln -s /usr/local/lib/libmcrypt.so /usr/lib/libmcrypt.so
ln -s /usr/local/lib/libmcrypt.so.4 /usr/lib/libmcrypt.so.4
ln -s /usr/local/lib/libmcrypt.so.4.4.8 /usr/lib/libmcrypt.so.4.4.8
ln -s /usr/local/lib/libmhash.a /usr/lib/libmhash.a
ln -s /usr/local/lib/libmhash.la /usr/lib/libmhash.la
ln -s /usr/local/lib/libmhash.so /usr/lib/libmhash.so
ln -s /usr/local/lib/libmhash.so.2 /usr/lib/libmhash.so.2
ln -s /usr/local/lib/libmhash.so.2.0.1 /usr/lib/libmhash.so.2.0.1
ln -s /usr/local/bin/libmcrypt-config /usr/bin/libmcrypt-config

tar zxvf "mhash-0.9.9.9.tar.gz"
cd mhash-0.9.9.9/
./configure
make && make install
cd ../

tar zxvf "mcrypt-2.6.8.tar.gz"
cd mcrypt-2.6.8/
./configure
make && make install
cd ../

tar zxvf "libpng-1.2.31.tar.gz"
cd libpng-1.2.31/
./configure
make && make install
cd ../


tar zxvf "jpegsrc.v6b.tar.gz"
cd jpeg-6b/
./configure
mkdir -p /usr/local/man/man1/
make && make install
cd ../

tar zxvf "autoconf-2.61.tar.gz"
cd autoconf-2.61/
./configure
make && make install
cd ../

tar zxvf "gd-2.0.35.tar.gz"
cd gd-2.0.35/
./configure
make && make install
cd ../

tar zxvf "libevent-2.0.21-stable.tar.gz"
./cofigure
make && make install
cd ../


tar zxvf "memcached-1.4.17.tar.gz"
cd memcached-1.4.17/
./configure \
--prefix=/usr/local/memcache
 make && make install
cd ../



#ldd /usr/local/memcache/bin/memcached
#ln -s /usr/local/lib/libevent-2.0.so.5 /usr/lib/libevent-2.0.so.5
ln -s /usr/lib64/libevent-2.0.so.5 /usr/lib/libevent-2.0.so.5

#/*
#-d选项是启动一个守护进程，
#-m是分配给Memcache使用的内存数量，单位是MB，我这里是10MB，
#-u是运行Memcache的用户，我这里是root，
#-l是监听的服务器IP地址，如果有多个地址的话，我这里指定了服务器的IP地址192.168.0.200，
#-p是设置Memcache监听的端口，我这里设置了12000，最好是1024以上的端口，
#-c选项是最大运行的并发连接数，默认是1024，我这里设置了256，按照你服务器的负载量来设定，
#-P是设置保存Memcache的pid文件，我这里是保存在 /tmp/memcached.pid
#*/
#pkill memcached


#tar zxvf "openssl-1.0.0l.tar.gz"
#cd openssl-1.0.0l/
#./configure
#make && make install
#cd ../

cd ../
