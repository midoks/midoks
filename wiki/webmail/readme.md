# webmail 搭建

- 源码地址:https://pan.baidu.com/s/1o7Tgmhk


# ready
```
yum -y install perl-FCGI
yum -y install perl-Unix-Syslog
yum -y install perl-DBD-MySQL
yum -y install perl-GD*
yum -y install perl-RRDs*
yum -y install perl-Time-HiRes perl-Time-HiRes-Value perl-File-Tail  rrdtool rrdtool-perl 


ln -s /tmp/mysql.sock /var/lib/mysql/mysql.sock

groupadd vmail
useradd -g vmail vmail
mkdir -p /home/domains/test.com/midoks/Maildir/
chown -R vmail:vmail /home/domains/test.com/midoks/Maildir/


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


yum -y install postfix
yum remove sendmail


#init mysql
mysql -u root -p < /var/www/extsuite/extman/docs/extmail.sql
mysql -u root -p < /var/www/extsuite/extman/docs/init.sql


cp -rf mailgraph_ext /usr/local/
/usr/local/mailgraph_ext/mailgraph-init start
/usr/local/mailgraph_ext/qmonitor start &
/var/www/extsuite/extman/daemon/cmdserver -d

```


# 参考
- http://ywzhou.blog.51cto.com/2785388/1590022