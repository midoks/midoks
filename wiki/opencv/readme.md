### OpenCV跨平台计算机视觉库学习

- http://wiki.opencv.org.cn
- http://opencv.org


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
