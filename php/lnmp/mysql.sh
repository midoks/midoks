
#! /bin/sh

#MySQL
cd ./source

groupadd mysql
useradd -g mysql mysql
tar zxvf "mysql-5.5.3-m3.tar.gz"
cd mysql-5.5.3-m3/
./configure --prefix=/usr/local/mysql \
--without-debug \
--enable-thread-safe-client \
--with-pthread \
--enable-assembler \
--enable-profiling \
--with-mysqld-ldflags=-all-static \
--with-client-ldflags=-all-static \
--with-extra-charsets=all \
--with-plugins=all \
--with-mysqld-user=mysql \
--without-embedded-server \
--with-server-suffix=-community \
--with-unix-socket-path=/tmp/mysql.sock
make && make install

#设置权限
setfacl -m u:mysql:rwx -R /usr/local/mysql
setfacl -m d:u:mysql:rwx -R /usr/local/mysql
#安装mysql和test数据库
/usr/local/mysql/bin/mysql_install_db --user=mysql
#启动mysql服务
#/usr/local/mysql/bin/mysqld_safe --user=mysql &
#修改mysql登录密码为123
#/usr/local/mysql/bin/mysqladmin -uroot password  123456789c
#用mysql登录
#/usr/local/mysql/bin/mysql -uroot -p123456789c

#ln -s /usr/lib64/mysql/ /usr/lib/mysql

cd ../
cd ../

#mysql socket 链接方式
#/usr/local/mysql/bin/mysql  -S /YOKA/DB/server0/mysql.sock -uroot -p -A