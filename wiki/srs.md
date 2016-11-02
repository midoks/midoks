# srs 安装
- https://github.com/ossrs/srs
```
./configure
make && make install
```

# srs 配置
```
ulimit -HSn 1107
./objs/srs -c conf/srs.conf
```