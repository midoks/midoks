
#一般性gd安装

#freetype
wget "http://download.savannah.gnu.org/releases/freetype/freetype-2.4.0.tar.bz2" 
tar jxvf freetype-2.4.0.tar.bz2
cd freetype-2.4.0
./configure --prefix=/usr/local/freetype && make && make install

#jpegsrc
wget "http://www.ijg.org/files/jpegsrc.v9.tar.gz" 
tar zxvf jpegsrc.v9.tar.gz
cd jpeg-9
CFLAGS="-O3 -fPIC" ./configure --prefix=/usr/local/jpeg && make && make install
mkdir -p /usr/local/jpeg/include
mkdir -p /usr/local/jpeg/lib
mkdir -p /usr/local/jpeg/bin
mkdir -p /usr/local/jpeg/man/man1

#libpng
wget "http://downloads.sourceforge.net/project/libpng/libpng12/1.2.50/libpng-1.2.50.tar.gz?r=http%3A%2F%2Fwww.libpng.org%2Fpub%2Fpng%2Flibpng.html&ts=1376631135&use_mirror=nchc"  
tar zxvf libpng-1.2.50.tar.gz
cd libpng-1.2.50 
CFLAGS="-O3 -fPIC" ./configure --prefix=/usr/local/libpng && make && make install

#gd.so

./configure --with-php-configure=/usr/local/php/bin/php-config  --with-jpeg-dir=/usr/local/jpeg  --with-png-dir=/usr/local/libpng   --with-freetype-dir=/usr/local/freetype
#或者
./configure --with-php-config=/usr/local/php/bin/php-config  --with-jpeg-dir=/usr/local/jpeg  --with-png-dir=/usr/local/libpng   --with-freetype-dir=/usr/local/freetype
make && make install
