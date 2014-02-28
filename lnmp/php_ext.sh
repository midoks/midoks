tar xvf "memcache-2.2.5.tgz"
cd memcache-2.2.5/
/usr/local/php/bin/phpize
./configure --enable-memcache \
-with-php-config=/usr/local/php/bin/php-config
make && make install