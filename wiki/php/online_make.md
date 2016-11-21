## 线上安装



#删除yum
rpm -aq | grep yum|xargs rpm -e --nodeps

## 更换yum源
wget -O /etc/yum.repos.d/CentOS-Base.repo http://mirrors.aliyun.com/repo/Centos-5.repo

http://mirror.centos.org/centos/
http://mirrors.aliyun.com/centos/

#安装yum
wget http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/python-iniparse-0.2.3-6.el5.noarch.rpm
wget http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/yum-metadata-parser-1.1.2-4.el5.x86_64.rpm
wget http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/yum-3.2.22-40.el5.centos.noarch.rpm  
wget http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/yum-fastestmirror-1.1.16-21.el5.centos.noarch.rpm
wget http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/python-elementtree-1.2.6-5.x86_64.rpm


wget http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/yum-3.2.29-73.el6.centos.noarch.rpm
wget http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/yum-metadata-parser-1.1.2-16.el6.x86_64.rpm
wget http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/python-urlgrabber-3.9.1-11.el6.noarch.rpm
wget http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/yum-plugin-fastestmirror-1.1.30-37.el6.noarch.rpm


yum clean all
yum makecache

yum groupinstall -y "Development Tools"

#php 稳定版
wget http://cn2.php.net/distributions/php-5.6.28.tar.gz

./configure --prefix=/usr/local/php5.6 \
--exec-prefix=/usr/local/php5.6 \
--with-config-file-path=/usr/local/php5.6 \
--with-iconv=/usr/local/libiconv \
--with-mysql=/usr/local/mysql \
--with-mysqli \
--enable-opcache \
--with-curl \
--enable-mbregex \
--enable-mbstring \
--with-mcrypt \
--enable-gd-native-ttf \
--with-openssl \
--with-mhash \
--enable-pcntl \
--enable-sockets \
--with-ldap \
--with-ldap-sasl \
--with-xmlrpc \
--enable-zip \
--enable-soap \
--enable-fpm \


wget http://ftp.gnu.org/pub/gnu/libiconv/libiconv-1.14.tar.gz
tar zxvf libiconv-1.14.tar.gz 
./configure --prefix=/usr/local/libiconv 
make && make install

#openresty
./configure --prefix=/usr/local/openresty \
--with-luajit \
--without-http_redis2_module \
--with-http_iconv_module \
--with-http_postgres_module \
--with-http_iconv_module \
--without-http_memcached_module \ 
--user=www \
--group=www \ 
--with-http_stub_status_module \
with-http_ssl_module
