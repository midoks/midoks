#! /bin/sh

#web dir
mkdir /web

#kenerl
cp -f ./conf/sysctl.min.conf /etc/sysctl.conf
sysctl -p

# For the first time you start
#nginx
cp -f ./conf/nginx.conf /usr/local/nginx/conf/nginx.conf
pkill -9 nginx
/usr/local/nginx/sbin/nginx -t
/usr/local/nginx/sbin/nginx -c /usr/local/nginx/conf/nginx.conf

#php
chmod  +x ./conf/php-fpm
cp -f ./conf/php-fpm /usr/local/php/php/fpm/php-fpm
cp -f ./conf/php-fpm.conf /usr/local/php/etc/php-fpm.conf
/usr/local/php/php/fpm/php-fpm start

#mysql
cp -f ./conf/my.cnf /etc/my.cnf
/usr/local/mysql/bin/mysqld_safe --user=mysql &
