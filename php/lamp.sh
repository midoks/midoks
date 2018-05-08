
cd /lamp/libxml2-2.6.30
./configure --prefix=/usr/local/libxml2/
make 
make install
 
cd /lamp/libmcrypt-2.5.8
./configure --prefix=/usr/local/libmcrypt/
make 
make install

cd /lamp/libmcrypt-2.5.8/libltdl
./configure --enable-ltdl-install
make
make install

cd /lamp/zlib-1.2.3
./configure
make
make install 

cd /lamp/libpng-1.2.31
./configure --prefix=/usr/local/libpng/
make
make install

mkdir /usr/local/jpeg6
mkdir /usr/local/jpeg6/bin
mkdir /usr/local/jpeg6/lib
mkdir /usr/local/jpeg6/include
mkdir -p /usr/local/jpeg6/man/man1
cd /lamp/jpeg-6b
./configure --prefix=/usr/local/jpeg6/ --enable-shared --enable-static
make
make install

cd /lamp/freetype-2.3.5
./configure --prefix=/usr/local/freetype/
make
make install

cd /lamp/autoconf-2.61
./configure
make 
make install
 
cd /lamp/gd-2.0.35
./configure --prefix=/usr/local/gd2/ --with-jpeg=/usr/local/jpeg6/ --with-freetype=/usr/local/freetype/
make
make install

cd /lamp/httpd-2.2.9
./configure --prefix=/usr/local/apache2/ --sysconfdir=/etc/httpd/ --with-included-apr --disable-userdir --enable-so --enable-deflate=shared --enable-expires=shared --enable-rewrite=shared --enable-static-support
make
make install
 
/usr/local/apache2/bin/apachectl start
echo "/usr/local/apache2/bin/apachectl start" >> /etc/rc.d/rc.sysinit

cd /lamp/ncurses-5.6
./configure --with-shared --without-debug --without-ada --enable-overwrite
make 
make install

groupadd mysql
useradd -g mysql mysql
cd /lamp/mysql-5.0.41
./configure --prefix=/usr/local/mysql/ --with-extra-charsets=all
make
make install

cp support-files/my-medium.cnf /etc/my.cnf
/usr/local/mysql/bin/mysql_install_db --user=mysql
chown -R root /usr/local/mysql
chown -R mysql /usr/local/mysql/var
chgrp -R mysql /usr/local/mysql

/usr/local/mysql/bin/mysqld_safe  --user=mysql &

cp /lamp/mysql-5.0.41/support-files/mysql.server /etc/rc.d/init.d/mysqld
chown root.root /etc/rc.d/init.d/mysqld
chmod 755 /etc/rc.d/init.d/mysqld
chkconfig --add mysqld
chkconfig --list mysqld
chkconfig --levels 245 mysqld off
 
cd /lamp/php-5.2.6
./configure --prefix=/usr/local/php/ --with-config-file-path=/usr/local/php/etc/ --with-apxs2=/usr/local/apache2/bin/apxs --with-mysql=/usr/local/mysql/ --with-libxml-dir=/usr/local/libxml2/ --with-jpeg-dir=/usr/local/jpeg6/ --with-freetype-dir=/usr/local/freetype/ --with-gd=/usr/local/gd2/ --with-mcrypt=/usr/local/libmcrypt/ --with-mysqli=/usr/local/mysql/bin/mysql_config --enable-soap --enable-mbstring=all --enable-sockets 
make
make install

cp php.ini-dist /usr/local/php/etc/php.ini
echo "Addtype application/x-httpd-php .php .phtml" >> /etc/httpd/httpd.conf
/usr/local/apache2/bin/apachectl restart



