### elastic启动问题

- 启动 elasticsearch 如出现异常  can not run elasticsearch as root
```
解决方法：创建ES 账户，修改文件夹 文件 所属用户 组
```


- ERROR: bootstrap checks failed
```

问题原因：因为Centos6不支持SecComp，而ES5.2.1默认bootstrap.system_call_filter为true进行检测，所以导致检测失败，失败后直接导致ES不能启动。

详见 ：https://github.com/elastic/elasticsearch/issues/22899

解决方法
在elasticsearch.yml中配置bootstrap.system_call_filter为false，注意要在Memory下面:

bootstrap.memory_lock: false
bootstrap.system_call_filter: false


vi /etc/security/limits.conf 
添加如下内容:
* soft nofile 65536
* hard nofile 131072
* soft nproc 2048
* hard nproc 4096
```

- unable to install syscall filter
```
使用心得linux版本，就不会出现此类问题了
```

- max number of threads [1024] for user [lish] likely too low, increase to at least [2048]
```
修改如下内容：
* soft nproc 1024
#修改为
* soft nproc 2048
```

- max virtual memory areas vm.max_map_count [65530] likely too low, increase to at least [262144]
```
解决：切换到root用户修改配置sysctl.conf
vi /etc/sysctl.conf 
添加下面配置：
vm.max_map_count=655360
并执行命令
sysctl -p
```