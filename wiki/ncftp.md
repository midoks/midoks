#ncftp

```
wget ftp://ftp.ncftp.com/ncftp/ncftp-3.2.6-src.tar.gz
tar zxvf ncftp-3.2.3-src.tar.gz cd ncftp-3.2.3/ 
./configure --prefix=/usr/local/ncftp 
make && make install
```

#CMD
```
-u：指定登录FTP服务器时使用的用户名； 
-p：指定登录FTP服务器时使用的密码； 
-P：如果FTP服务器没有使用默认的TCP协议的21端口，则使用此选项指定FTP服务器的端口号。
-m：在传之前尝试在目录位置创建目录(用于传目录的情况) -R：递规传子目录

```
