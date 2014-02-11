#! /bin/sh

/usr/local/nginx/sbin/nginx -c /usr/local/nginx/conf/nginx.conf
/usr/local/php/php/fpm/php-fpm start
/usr/local/mysql/bin/mysqld_safe --user=mysql &
