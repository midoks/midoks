### CSVN 安装
- https://www.collab.net/downloads
- wget https://downloads-guests.open.collab.net/files/documents/61/17071/CollabNetSubversionEdge-5.2.0_linux-x86_64.tar.gz

- 需要安装JAVA环境
```
vi /etc/profile
export JAVA_HOME=/usr/lib/jvm/java-1.8.0-openjdk-1.8.0.131-0.b11.el6_9.x86_64
export PATH=$JAVA_HOME/bin:$PATH
export CLASSPATH=.:$JAVA_HOME/lib/dt.jar:$JAVA_HOME/lib/tools.jar
source /etc/profile

tar -zxvf CollabNetSubversionEdge-5.2.0_linux-x86_64.tar.gz
useradd csvn
mv csvn/ /
chown -R  csvn:csvn /csvn/


ln -s /csvn/csvn/bin/csvn /etc/csvn
ln -s /csvn/csvn-httpd /etc/csvn-httpd

```



- 启动服务：csvn csvn-httpd
```
service csvn start
service csvn-httpd start

```

- 地址访问
```
http://ip:3343
admin
admin
```