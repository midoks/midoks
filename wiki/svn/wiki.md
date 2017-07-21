### 基本配置
- https://github.com/midoks/midoks.github.io/blob/master/_posts/%E8%BF%90%E7%BB%B4/2014-5-28-svn.md
- svn info


### csvn
http://www.collab.net/products/integrations/certified

```
tar.gz:CollabNetSubversionEdge-5.1.0_linux-x86_64.tar.gz

#查看svn URL地址
svn info | grep URL

groupadd svn 
useradd -g svn svnuser
passwd svnuser

#授权
chown svnuser:root -R csvn/
sudo su - svnuser

#启动
cd csvn/bin
./csvn start
#service csvn stop

//增加用户
#../csvn/bin/htpasswd  -b  ../csvn/data/conf/svn_auth_file test test 
Automatically using MD5 format.
Adding password for user test
 
//删除用户
#../csvn/bin/htpasswd -D  ../csvn/data/conf/svn_auth_file test
Automatically using MD5 format.
Deleting password for user test

//修改密码
#../csvn/bin/htpasswd -D ../csvn/data/conf/svn_auth_file test
#../csvn/bin/htpasswd -b ../csvn/data/conf/svn_auth_file test 1234
```

# 语法检查
svnlook cat /u01/svndata/svnroot/repo index.php | php -l

- 参考
http://www.it165.net/os/html/201401/7096.html