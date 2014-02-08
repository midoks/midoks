#MySQL
groupadd mysql
useradd -g mysql mysql
tar zxvf mysql-5.5.3-m3.tar.gz
cd mysql-5.5.3-m3/
./configure --prefix=/usr/local/mysql \
--without-debug \
--enable-thread-safe-client \
--with-pthread --enable-assembler \
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

cp /usr/local/mysql/share/mysql/my-medium.cnf /etc/my.cnf
yum install acl
setfacl -m u:mysql:rwx -R /usr/local/mysql
setfacl -m d:u:mysql:rwx -R /usr/local/mysql
#设置权限
/usr/local/mysql/bin/mysql_install_db --user=mysql
#安装mysql和test数据库
/usr/local/mysql/bin/mysqld_safe --user=mysql &
#启动mysql服务
/usr/local/mysql/bin/mysqladmin -uroot password  149012cjs
#修改mysql登录密码为123
/usr/local/mysql/bin/mysql -uroot -p149012cjs
#用mysql登录
ln -s /usr/lib64/mysql/ /usr/lib/mysql