### 日志分析系统部署

aliyun
```
yum -y install zlib
```


- openresty
```
pcre https://sourceforge.net/projects/pcre/files/latest/download
./configure --prefix=/usr/local/pcre && make && make install

wget http://www.openssl.org/source/openssl-1.0.0a.tar.gz
./config && make && make install

wget https://openresty.org/download/openresty-1.11.2.1.tar.gz
./configure --prefix=/usr/local/openresty  --with-pcre=/package/pcre-8.38 --with-openssl=/package/openssl-1.0.0a
gmake
gmake install

##启动
/usr/local/openresty/nginx/sbin/nginx -c /usr/local/openresty/nginx/conf/nginx.conf
## 停止|重启
/usr/local/openresty/nginx/sbin/nginx -s relaod|stop
##检查
/usr/local/openresty/nginx/sbin/nginx -t

#下载kafka lua组件
wget https://github.com/doujiang24/lua-resty-kafka/archive/v0.05.tar.gz

```


- java install
http://www.oracle.com/technetwork/java/javase/downloads/jdk8-downloads-2133151.html
```
rpm -qa|grep java
rpm -e --nodeps tzdata-java-2012c-1.el6.noarch  
rpm -e --nodeps java-1.7.0-openjdk-1.7.0.45-1.45.1.11.1.el6.x86_64

#重新安装
rpm -ivh jdk-8u101-linux-x64.rpm


```


- kafka
http://kafka.apache.org/


```
tar zvxf kafka_2.10-0.10.0.1.tgz

#cmd
./kafka-topics.sh --create --zookeeper 127.0.0.1:2181 --topic test --replication-factor 1 --partition 1

#开启配置
listeners=PLAINTEXT://127.0.0.1:9092


```

- logstash
https://www.elastic.co/products/logstash
http://kibana.logstash.es/content/logstash/scale/redis.html
```
tar zxvf logstash-2.3.4.tar.gz

#插件安装
bin/logstash-plugin install logstash-input-kafka
bin/logstash-plugin install logstash-output-kafka

bin/logstash-plugin install logstash-input-elasticsearch
bin/logstash-plugin install logstash-output-elasticsearch

bin/logstash-plugin install logstash-input-redis
bin/logstash-plugin install logstash-output-redis

配置文件

常用命令
/usr/local/logstash/bin/logstash -f /usr/local/logstash/logstash.conf


```


- elasticsearch
https://www.elastic.co/
https://www.elastic.co/downloads/marvel
http://www.learnes.net/

#集群配置
http://blog.csdn.net/he90227/article/details/49782145
```
bin/plugin install mobz/elasticsearch-head
bin/plugin install elasticsearch/marvel/latest
bin/plugin install license
```
http://139.129.217.129:9200/_plugin/head/


