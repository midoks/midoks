tar zxvf pcre-8.10.tar.gz
cd pcre-8.10/
./configure
make && make install
cd ../

echo "Nginx install"
tar zxvf nginx-1.5.9.tar.gz
cd nginx-1.5.9/
./configure \
--user=nginx \
--group=nginx \
--prefix=/usr/local/nginx \
--with-http_stub_status_module \
--with-http_ssl_module \
--with-http_realip_module \
--with-http_image_filter_module
make && make install
cd ../