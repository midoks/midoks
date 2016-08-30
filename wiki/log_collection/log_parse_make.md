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

#下载kafka lua组件
wget https://github.com/doujiang24/lua-resty-kafka/archive/v0.05.tar.gz

```