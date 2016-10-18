### redis 安装
```
wget http://pecl.php.net/get/redis-2.2.8.tgz

wget http://download.redis.io/releases/redis-2.8.12.tar.gz

make install

cp redis.conf /etc/


make install命令执行完成后，会在/usr/local/bin目录下生成本个可执行文件，分别是redis-server,redis-cli,redis-benchmark,redis-check-aof,redis-check-dump,它们的作用如下:

redis-server：Redis服务器的daemon启动程序
redis-cli：Redis命令行操作工具。也可以用telnet根据其纯文本协议来操作
redis-benchmark：Redis性能测试工具，测试Redis在当前系统下的读写性能
redis-check-aof：数据修复
redis-check-dump：检查导出工具

#启动
/usr/local/bin/redis-server  /usr/local/redis/redis.cc.conf
```              