### openresty 使用

### 需要的资源
- wget http://openresty.org/download/ngx_openresty-1.4.3.6.tar.gz
- http://www.pcre.org/

### 安装
```
yum install readline-devel pcre-devel openssl-devel gcc tcl

unzip pcre-8.38.zip 
./configure --enable-utf8

tar zxvf openresty-1.9.7.4.tar.gz

./configure \
--prefix=/usr/local/openresty \
--with-luajit \
--with-pcre=../pcre \
--without-http_redis2_module \
--with-http_iconv_module \
--with-http_postgres_module \
--with-debug 


make
make install


make -j2 #支持多核特性

##启动
/usr/local/openresty/nginx/sbin/nginx -c /usr/local/openresty/nginx/conf/nginx
## 停止|重启
/usr/local/openresty/nginx/sbin/nginx -s relaod|stop
##检查
/usr/local/openresty/nginx/sbin/nginx -t

```



### 注意
--with-pcre=../pcre \ #必须填写 pcre包解药路径

开发模式,把次配置关闭
lua_code_cache off;