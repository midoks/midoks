# webmail 搭建

- 源码地址:https://pan.baidu.com/s/1o7Tgmhk


# ready
```
yum -y  install perl-FCGI
yum install -y perl-Unix-Syslog
yum -y install perl-DBD-MySQL

ln -s /tmp/mysql.sock /var/lib/mysql/mysql.sock



```

# install 

```
groupadd vgroup
useradd -g vuser vgroup


tar -zxvf extmail-1.2.tar.gz 
tar -zxvf extman-1.1.tar.gz

mkdir -p /var/www/extsuite
mv extmail-1.2/ /var/www/extsuite/extmail
mv extman-1.1/ /var/www/extsuite/extman


```