#PHP
echo "PHP install"

cd ./source

tar jxvf "php-5.5.7.tar.bz2"
cd php-5.5.7/
./configure --prefix=/usr/local/php \
--exec-prefix=/usr/local/php/ \
--with-config-file-path=/usr/local/php/etc \
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
--enable-fpm
make && make install
cd ../

cd ../
