## 线上安装

-(rpm修复) http://rpm5.org/
- https://github.com/rpm-software-management/rpm
- wget http://ftp.rpm.org/releases/rpm-4.4.x/rpm-4.4.2.3.tar.gz
```

./configure --prefix=/usr \
--build=x86_64-linux \
--host=x86_64 \
CPPFLAGS="-I/usr/include/nspr4/ -I/usr/include/nss3 -I/usr/include/linux -I/usr/include/ -std=gnu99" \
CC="gcc -std=gnu99" 

cp /usr/lib/libz.so /usr/local/lib/
--force --nodeps 


rpm -ivh http://mirrors.163.com/centos/5/os/i386/CentOS/openssl-devel-0.9.8e-20.el5.i386.rpm \
http://mirrors.163.com/centos/5/os/i386/CentOS/openssl-0.9.8e-20.el5.i386.rpm


http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/python-sqlite-1.1.7-1.2.1.x86_64.rpm --force --nodeps 

rpm -ivh http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/rpm-python-4.4.2.3-34.el5.x86_64.rpm --force --nodeps

yum install http://ftp.sjtu.edu.cn/fedora/epel/6/x86_64/epel-release-6-8.noarch.rpm
```


#删除yum
```
rpm -aq | grep yum | xargs rpm -e --nodeps
rpm -aq | grep python-iniparse | xargs rpm -e --nodeps
rpm -aq | grep python-urlgrabber | xargs rpm -e --nodeps

yum install lrzsz
```

#YUM安装 (redhat5)
```
rpm -ivh http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/python-iniparse-0.2.3-6.el5.noarch.rpm 
rpm -ivh http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/yum-metadata-parser-1.1.2-4.el5.x86_64.rpm

rpm -ivh http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/yum-3.2.22-40.el5.centos.noarch.rpm \
http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/python-urlgrabber-3.1.0-6.el5.noarch.rpm \
http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/yum-fastestmirror-1.1.16-21.el5.centos.noarch.rpm \
http://mirrors.aliyun.com/centos/5/os/x86_64/CentOS/yum-metadata-parser-1.1.2-4.el5.x86_64.rpm

wget -O /etc/yum.repos.d/rhel-source.repo http://mirrors.aliyun.com/repo/Centos-6.repo
sed -i 's/$releasever/5/g' rhel-source.repo
```


#YUM安装 (redhat6)
```
rpm -ivh http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/python-iniparse-0.3.1-2.1.el6.noarch.rpm
rpm -ivh http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/yum-metadata-parser-1.1.2-16.el6.x86_64.rpm

rpm -ivh http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/yum-3.2.29-81.el6.centos.noarch.rpm \
http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/yum-plugin-fastestmirror-1.1.30-40.el6.noarch.rpm  \
http://mirrors.aliyun.com/centos/6/os/x86_64/Packages/python-urlgrabber-3.9.1-11.el6.noarch.rpm
```

#添加源
```
cd /etc/yum.repos.d/
rm -rf *
wget -O /etc/yum.repos.d/rhel-source.repo http://mirrors.aliyun.com/repo/Centos-6.repo
sed -i 's/$releasever/6/g' rhel-source.repo
```

#安装java1.8
```
yum -y install java-1.8.0-openjdk
```


#other
```
yum clean all
yum makecache

yum install ca-certificates
yum groupinstall -y "Development Tools"
package-cleanup --cleandupes --setopt=protected_multilib=false --setopt=protected_multilib=false

rpm -Uvh yum* --nodeps
rpm --rebuilddb

rpm -ql rpm
rpm -ql rpm-libs
```

#php 稳定版

```

yum install -y ImageMagick* libxml2*
yum install -y libmcrypt libmcrypt-devel mcrypt mhash curl curl-devel
yum install -y libpng libpng-devel libjpeg-turbo libjpeg-turbo-devel freetype freetype-devel gd



wget http://ftp.gnu.org/pub/gnu/libiconv/libiconv-1.14.tar.gz
tar zxvf libiconv-1.14.tar.gz 
./configure --prefix=/usr/local/libiconv 
make && make install


wget http://cn2.php.net/distributions/php-5.6.28.tar.gz
wget http://cn2.php.net/distributions/php-7.0.14.tar.gz
wget http://cn2.php.net/distributions/php-7.1.6.tar.gz


yum install -y openssl-devel libxml2

./configure --prefix=/usr/local/php56 \
--exec-prefix=/usr/local/php56 \
--with-config-file-path=/usr/local/php56/etc \
--with-iconv=/usr/local/libiconv \
--with-mysql=/usr/local/mysql \
--with-pdo-mysql \
--with-mysql-sock=/var/mysql/mysql.sock \
--with-mysqli \
--enable-opcache \
--with-curl \
--with-mcrypt \
--enable-mbregex \
--enable-mbstring \
--enable-gd-native-ttf \
--with-openssl \
--with-mhash \
--enable-pcntl \
--enable-sockets \
--with-xmlrpc \
--enable-zip \
--enable-soap \
--enable-fpm \
--with-gd \
--with-jpeg-dir \
--with-png-dir \
--with-freetype-dir 

./configure --prefix=/usr/local/php71 \
--exec-prefix=/usr/local/php71 \
--with-config-file-path=/usr/local/php71/etc \
--with-mysqli \
--enable-opcache \
--with-curl \
--enable-mbregex \
--enable-mbstring \
--enable-gd-native-ttf \
--with-openssl \
--with-mhash \
--enable-pcntl \
--enable-sockets \
--with-xmlrpc \
--enable-zip \
--enable-soap \
--enable-fpm \
--with-gd \
--with-jpeg-dir \
--with-png-dir \
--with-freetype-dir 



./configure --prefix=/usr/local/php70 \
--exec-prefix=/usr/local/php70 \
--with-config-file-path=/usr/local/php70/etc \
--enable-opcache \
--enable-fpm \
--without-iconv \
--disable-fileinfo \
--enable-debug

--enable-dtrace \
--with-mcrypt \
--with-ldap \
--with-ldap-sasl \

```

# nginx

```
yum -y install pcre-devel openssl openssl-devel
wget http://nginx.org/download/nginx-1.10.3.tar.gz
wget http://nginx.org/download/nginx-1.13.5.tar.gz

./configure \
--prefix=/usr/local/nginx


./configure \
--prefix=/usr/local/nginx \
--with-http_stub_status_module \
--with-http_ssl_module


```

# openresty

```




wget http://openresty.org/download/openresty-1.11.2.2.tar.gz

./configure \
--prefix=/usr/local/openresty \
--with-pcre=/usr/local/pcre \
--with-luajit \
--with-http_postgres_module \
--user=www \
--group=www \
--with-http_stub_status_module \
--with-http_ssl_module \
--disable-shared \

./configure \
--prefix=/usr/local/openresty \
--with-luajit \
--without-http_redis2_module \
--with-http_iconv_module \
--with-http_postgres_module \  
--without-http_memcached_module \
--user=www \
--group=www \
--with-http_stub_status_module \
--with-http_ssl_module \
```

#memcached
```
yum install memcached
memcached -d -m 256 -u root -p 11211 -c 1024 –P /tmp/memcached.pid
```
