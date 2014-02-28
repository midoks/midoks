#! /bin/sh

/usr/local/nginx/sbin/nginx -c /usr/local/nginx/conf/nginx.conf
/usr/local/php/php/fpm/php-fpm start
/usr/local/mysql/bin/mysqld_safe --user=mysql &

/usr/local/memcache/bin/memcached -d -m 100 -l "127.0.0.1" -p 11211 -u "root"
