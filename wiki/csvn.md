### CSVN 安装
- https://www.collab.net/downloads
- wget https://downloads-guests.open.collab.net/files/documents/61/17071/CollabNetSubversionEdge-5.2.0_linux-x86_64.tar.gz

- 需要安装JAVA环境
```
vi /etc/profile
export JAVA_HOME=/usr/lib/jvm/java-1.8.0-openjdk-1.8.0.131-0.b11.el6_9.x86_64
export PATH=$JAVA_HOME/bin:$PATH
export CLASSPATH=.:$JAVA_HOME/lib/dt.jar:$JAVA_HOME/lib/tools.jar
source /etc/profile

tar -zxvf CollabNetSubversionEdge-5.2.0_linux-x86_64.tar.gz
useradd csvn
mv csvn/ /
chown -R  csvn:csvn /csvn/


ln -s /csvn/csvn/bin/csvn /etc/csvn
ln -s /csvn/csvn-httpd /etc/csvn-httpd

```



- 启动服务：csvn csvn-httpd
```
service csvn start
service csvn-httpd start

```

- 地址访问
```
http://ip:3343
admin
admin
```


## 自动化配置
```
- 通过sh,可以svn代码
vi /home/www/.subversion/servers
store-passwords = yes
store-plaintext-passwords = yes

- 设置权限
visudo
www     ALL=(ALL)       ALL
chown -R www:www /var/www

```

## svn hook (post-commit)
```
#!/bin/sh


export LC_ALL=en_US.UTF-8
export LC_CTYPE=zh_CN.UTF-8

REPOS="$1"
REV="$2"

REPOS_NAME=${REPOS##*/}

WEB_PATH=/var/www/$REPOS_NAME
SVN_PATH=http://127.0.0.1:18080/svn/$REPOS_NAME

if [ ! -d  $WEB_PATH ]; then
	svn co $SVN_PATH $WEB_PATH --username=admin --password=admin
fi

cd $WEB_PATH
svn update --username admin --password admin
```

## svn hook (pre-commit)
```
#!/bin/sh

export LC_ALL=en_US.UTF-8
export LC_CTYPE=zh_CN.UTF-8

#!/bin/sh


REPOS="$1"
TXN="$2"

REPOS_NAME=${REPOS##*/}
WEB_PATH=/var/www/$REPOS_NAME
NGINX_PATH=/usr/local/nginx

SVNLOOK=/usr/bin/svnlook


/usr/local/php71/bin/php /var/www/php_config_yaml/parse.php $NGINX_PATH $WEB_PATH $1 $2
```


### FAQ
```
普通用户在restart和reload nginx时:
会报错：the "user" directive makes sense only if the master process runs with super-user privileges

我又不能给开发人员root权限，没办法，只好这么做。

原因是:默认情况下linux的1024以下端口是只有root用户才有权限占用
方法一:

所有用户都可以运行（因为是755权限，文件所有者：root，组所有者：root）
chown root:root nginx
chmod 755 nginx
chmod u+s nginx

方法二:
仅 root 用户和 www 用户可以运行（因为是750权限，文件所有者：root，组所有者：www）
chown root:www nginx
chmod 750 nginx
chmod u+s nginx
```

