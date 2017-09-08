### window编译php及php扩展

- https://wiki.php.net/internals/windows/stepbystepbuild
- http://windows.php.net/downloads/php-sdk/
- https://github.com/php-memcached-dev/php-memcached
- http://www.cygwin.com/
- http://gnuwin32.sourceforge.net/packages/make.htm
- http://svn.coderepos.org/share/lang/c/libmemcached-win32/
- https://github.com/infusion/PHP-Facedetect


```
php ext_skel_win32.php --extname=kkk

phpsdk_buildtree.bat phpdev

phpsdk_setvars.bat

cd phpdev/vc14/x64

buildconf --force

buildconf
configure --help
configure --disable-all --enable-cli
nmake

nmake snap

configure --disable-all --enable-cli --enable-xhprof=shared
configure --disable-all --enable-cli --enable-md_xhprof=shared
configure --disable-all --enable-cli --with-opencv=shared
configure --disable-all --enable-cli --with-facedetect=shared


echo %lib%
set LIB=%LIB%;D:\opencv3\opencv\build\x64\vc14\lib
set LIB=%LIB%;D:\opencv\opencv\build\x64\vc12\lib

echo %PATH%
set PATH=%PATH%;D:\opencv3\opencv\build\include



configure --disable-all --enable-cli --enable-memcached=shared --with-memcached=D:/MD/php_sdk/libmemcached-win32/libmemcached-latest/libmemcached/

configure --disable-all --enable-cli --with-memcached=D:/MD/php_sdk/libmemcached-win32/libmemcached-latest/libmemcached/memcached.dll


configure --disable-all --enable-cli --enable-apache2-4handler --without-mssql --without-pdo-mssql --disable-isapi --enable-com-dotnet=shared --with-mcrypt=static --disable-static-analyze --with-pgo

```


- 遇到的问题

- 1.命令缺少(mc)
从（ vs2012 | vs2015 ）命令行进入,不会遇到此问题( vs2012 | vs2015 ->工具->Visual Studio命令提示(C) )
```
Recreating build dirs
        "" -h win32\ -r x64\Release_TS\ -x x64\Release_TS\ win32\build\wsyslog.m
c
'-h' 不是内部或外部命令，也不是可运行的程序
或批处理文件。
NMAKE : fatal error U1077: “"”: 返回代码“0x1”


C:/Program Files (x86)/Windows Kits/8.0/bin/x64/mc.exe(路径加入到系统变量中)
```
- 环境64,32
```
C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64\vcvars64.bat
复制到相应的位置上

```
### 相关文章
- http://blog.csdn.net/letshi/article/details/45702299

- 如有错误及不足,欢迎指正