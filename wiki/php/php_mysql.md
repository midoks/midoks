### mysql5.7.12 安装

- wget http://dev.mysql.com/get/Downloads/MySQL-5.7/mysql-5.7.12.tar.gz


- 安装过程
```
cmake \
-DCMAKE_INSTALL_PREFIX=/usr/local/mysql \
-DMYSQL_DATADIR=/usr/local/mysql/data \
-DSYSCONFDIR=/etc \
-DWITH_MYISAM_STORAGE_ENGINE=1 \
-DWITH_INNOBASE_STORAGE_ENGINE=1 \
-DWITH_MEMORY_STORAGE_ENGINE=1 \
-DWITH_READLINE=1 \
-DMYSQL_UNIX_ADDR=/var/lib/mysql/mysql.sock \
-DMYSQL_TCP_PORT=3306 \
-DENABLED_LOCAL_INFILE=1 \
-DWITH_PARTITION_STORAGE_ENGINE=1 \
-DEXTRA_CHARSETS=all \
-DDEFAULT_CHARSET=utf8 \
-DDEFAULT_COLLATION=utf8_general_ci


cmake \
-DCMAKE_INSTALL_PREFIX=/usr/local/mysql57 \
-DMYSQL_DATADIR=/usr/local/mysql57/data \
-DSYSCONFDIR=/usr/local/mysql57 \
-DWITH_MYISAM_STORAGE_ENGINE=1 \
-DWITH_INNOBASE_STORAGE_ENGINE=1 \
-DWITH_MEMORY_STORAGE_ENGINE=1 \
-DWITH_READLINE=1 \
-DMYSQL_UNIX_ADDR=/var/tmp/mysql.sock \
-DMYSQL_TCP_PORT=3306 \
-DENABLED_LOCAL_INFILE=1 \
-DWITH_PARTITION_STORAGE_ENGINE=1 \
-DEXTRA_CHARSETS=all \
-DDEFAULT_CHARSET=utf8 \
-DDEFAULT_COLLATION=utf8_general_ci \
-DDOWNLOAD_BOOST=1 \
-DWITH_BOOST=/usr/local/boost


groupadd mysql
useradd -g mysql mysql
设置权限并初始化MySQL系统授权表
修改/usr/local/mysql权限
chown -R mysql:mysql /usr/local/mysql

#初始化数据
./bin/mysqld --initialize

#启动(与关闭)
./bin/mysqld_safe --user=mysql &
./bin/mysqladmin -uroot shutdown

#修改密码
SET PASSWORD = PASSWORD('123456');
flush privileges;

update user set host = '%' where user = 'root';

#授权登录
grant all privileges on *.* to 'yourname'@'%' identified by 'youpasswd';

     
```



