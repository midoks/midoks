### OpenCV跨平台计算机视觉库学习

- http://wiki.opencv.org.cn
- http://opencv.org

### Linux 安装 (2.4.9)

```
yum -y install gtk+ gtk+-devel pkgconfig libpng zlib libjpeg libtiff cmake

# 下载
wget https://github.com/opencv/opencv/archive/2.4.9.tar.gz

# 解压安装包
tar xvf 2.4.9.tar.gz

# 进入opencv目录
cd opencv-2.4.9/

# 编译安装
cmake CMakeLists.txt
make -j $(cat /proc/cpuinfo|grep processor|wc -l)
make install
```


### Window 配置

### MACOSX XCODE配置

- 基础安装
```
ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
brew install wget
brew install cmake
brew install opencv
```

- 创建xcode项目（Command Line)
- 添加库文件
```
Build Phases -> Link Binary With Libraries -> + (shift+command+g [/usr/local/Cellar/opencv/版本/lib])
```

- 添加文件头(SEARCH PATHS)
```
Build Settings -> (在搜索框输入 SEARCH PATHS)

-> Header Search Paths    添加  /usr/local/include
-> Library Search Path    添加  /usr/local/lib
```

## 参考
- http://blog.csdn.net/u014365862/article/details/53067565
