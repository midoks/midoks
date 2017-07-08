# 配置
```
vi /etc/rsyncd.conf
```

# 例子
```
uid = www
gid = www
use chroot = no
max connections = 100
log file = /var/log/rsyncd.log
pid file = /var/run/rsyncd.pid
list = false

#项目名
[solr_service]
path = /var/www/solr_service
read only = false
hosts allow = 10.0.10.10 10.0.10.123

```

# 常用命令
```
useradd www
/usr/bin/rsync --daemon
/etc/rc.d/rc.local
```