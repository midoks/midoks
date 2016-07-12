### mongo 安装
```
yum install cyrus-sasl-devel
pecl mongo install

curl -O https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-3.0.6.tgz
tar -zxvf mongodb-linux-x86_64-3.0.6.tgz                             
mv  mongodb-linux-x86_64-3.0.6/ /usr/local/mongodb

sudo /usr/local/mongodb/bin/mongod -f /usr/local/mongodb/mongodb.conf


        
```              