### memcached 安装
```
wget http://www.monkey.org/~provos/libevent-2.0.12-stable.tar.gz
tar xzfv libevent-2.0.12-stable.tar.gz
cd libevent-2.0.12-stable

./configure && make && make install

wget http://www.memcached.org/files/memcached-1.4.30.tar.gz
tar xzfv memcached-1.4.5.tar.gz 
cd memcached-1.4.5

./configure --prefix=/usr/local/memcached && make && make install

./memcached -d -u nobody -m 512 127.0.0.1 -p 11211

```


### php memcached 扩展安装
```
wget https://launchpadlibrarian.net/51546357/libmemcached-0.42.tar.gz
tar zxvf libmemcached-0.42.tar.gz
./configure --prefix=/usr/local/libmemcached  --with-memcached && make && make install

```


### PHP memcache分布式问题

- memcache










