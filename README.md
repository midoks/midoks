### 《记录我的代码片段》

Record my code snippet

### 我的相关地址
- [我的博客](http://midoks.cachecha.com)
- [我的微博](http://weibo.com/u/1504761980)


### c10k && c10m
```
http://openresty.org/en/

http://dpdk.org/download

```

##vpn
```
wget --no-check-certificate https://raw.githubusercontent.com/teddysun/across/master/l2tp.sh
chmod +x l2tp.sh
./l2tp.sh
```

### 常用
```
sublime 分屏 切换快捷键
- ctrl+shift+2,3,4,5
- ctrl+tab

vim 分屏 切换快捷键
- vim -o3 f1 f2 f3
- sp file  	水平分屏
- vsp file   垂直分屏
- ctrl + w + w
- ctrl + w + h  左
- ctrl + w + l  右
```

### sublime phpfmt 设置
```
{
	"version": 1,
	"enable_auto_align": true,//自动调整对齐
	"indent_with_space": true,//自动空格，tab不会出现
	"psr4": true,
	"php_bin":"~~/php.exe",//环境没有配置php时，需要添加此项，指向php.exe
}

安装插件Package Control
View -> Show Console（或者使用快捷键 command + ` ）

https://packagecontrol.io/installation
```
### php语法检车

```
find ./app/ -name "*.php" | xargs -n 1 /usr/local/product/php-5.5.18/bin/php -l
```

## mac删除@权限
```
xattr -c *
```

## 设置服务器时区
```
echo "TZ='Asia/Beijing'; export TZ" >> /etc/profile; source /etc/profile
```

### 常用跟踪调试
```

find /data -type f -size +100M  -print0 | xargs -0 du -h | sort -nr

#*uix
top -p `pidof php-fpm | sed 's/[[:space:]]/,/g'`
#mac
top `pidof php-fpm | sed 's/\([0-9]* \)/-pid \1/g'`


yum install -y strace

# vim /root/.bashrc 	//*uix
source /root/.bashrc
function straceall {
echo "strace $(pidof "${1}" | sed 's/\([0-9]*\)/-p \1/g')"
strace $(pidof "${1}" | sed 's/\([0-9]*\)/-p \1/g')
}


function straceallr {
echo "strace $(ps -ef|grep ${1} | grep -v grep | awk '{print $2}' |sed 's/\([0-9]*\)/-p \1/g'| tr '\n' ' ')"
strace $(ps -ef|grep ${1} | grep -v grep | awk '{print $2}' |sed 's/\([0-9]*\)/-p \1/g' tr '\n' ' ')
}


 

brew install pidof
brew install dtruess
# vim ~/.bash_profile 	//mac
source /root/.bashrc
function straceall {
echo "sudo dtruss $(pidof "${1}" | sed 's/\([0-9]* \)/-p \1/g')"
sudo dtruss $(pidof "${1}" | sed 's/\([0-9]* \)/-p \1/g')
}

sed '/^ *#/d' **.conf > *.bak.conf

```

### iptables
```
iptables -A INPUT -p tcp -s 218.247.181.51 -j ACCEPT

#删除规则
iptables -D INPUT 2 

```