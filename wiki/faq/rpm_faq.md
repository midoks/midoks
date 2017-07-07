# 解决rpm 命令不能用

```
rpm -qa
killall -9 rpm

cd /var/lib/rpm/

rm -f __db.00*
rpm --rebuilddb
rpm -qa|grep conf
```