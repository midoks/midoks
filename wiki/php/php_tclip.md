### tclip 安装

- http://wiki.opencv.org.cn/index.php/Download
- https://github.com/exinnet/tclip


- MacOSX 安装过程
```
brew install gtk+  pkgconfig libpng lzlib libjpeg libtiff cmake

sudo chown -R "$USER":admin /usr/local
mkdir -p /Library/Caches/Homebrew
sudo chown -R "$USER":admin /Library/Caches/Homebrew
     
```

- opencv安装
 * cd 进入安装包文件夹内。
 * cmake CMakeLists.txt
 * make && make install
 * vim /etc/profile
 * 在 unset i 前增加
 * export PKG_CONFIG_PATH=/usr/lib/pkgconfig/:/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH
 * 保持退出后，执行如下命令source /etc/profile
 * echo "/usr/local/lib/" > /etc/ld.so.conf.d/opencv.conf
 * ldconfig

- 安装php图片裁剪tclip扩展
 * cd 到源代码目录中的php_ext文件夹
 * phpize
 * ./configure
 * make
 * cp modules/tclip.so 到 extension 目录
 * 修改php.ini。加入 extension=tclip.so
 * 重启fpm
