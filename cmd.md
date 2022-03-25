### 《记录我的代码片段》

Record my code snippet

### 我的相关地址
- [我的博客](http://www.cachecha.com)
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

##bbr
```
wget --no-check-certificate https://github.com/teddysun/across/raw/master/bbr.sh
chmod +x bbr.sh
./bbr.sh
```

##ssh代理
```
ssh -D 8081 root@ip
socket 127.0.0.1 8081
```

```
echo  "Host *\n\tServerAliveInterval 60" > ~/.ssh/config
```

### 常用

- sudo spctl --master-disable
这条命令作用就是使得mac运行安装任何来源App,不然的话，安装破解的cornerstone会显示文件损坏

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

composer install --ignore-platform-reqs
```
### php语法检车

```
find ./app/ -name "*.php" | xargs -n 1 /usr/local/product/php-5.5.18/bin/php -l
```

### 杀死僵尸进程
```
ps -A -o stat,ppid,pid,cmd | grep -e '^[Zz]' | awk '{print $2}' | xargs kill -9
```

### PHPINFO
```
php -r "phpinfo();"
```

### iftop流量查看
```
iftop -i eth2 -n  -P
```

## mac删除@权限
```
xattr -c *
find . -type d -name ".svn"|xargs rm -rf
```

## mac删除.DS_Store
```
find . -name .DS_Store -print0 | xargs -0 git rm -f --ignore-unmatch
```

## 设置服务器时区
```
echo "TZ='Asia/Beijing'; export TZ" >> /etc/profile; source /etc/profile
```

## 磁盘格式化
```
mkfs.ext4 /dev/sdb
mount -t ext4 /dev/sdb /home
```

## vps [openvz|kvm]
```
yum -y install virt-what && virt-what
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
strace $(ps -ef|grep ${1} | grep -v grep | awk '{print $2}' |sed 's/\([0-9]*\)/-p \1/g' | tr "\n" " ")
}

mac 
function straceallr {
echo "sudo struss $(ps -ef|grep ${1} | grep -v grep | awk '{print $2}' |sed 's/\([0-9]*\)/-p \1/g'| tr '\n' ' ')"
sudo struss $(ps -ef|grep ${1} | grep -v grep | awk '{print $2}' |sed 's/\([0-9]*\)/-p \1/g' | tr "\n" " ")
}

strace -T -e clone $(ps -ef|grep gearmand | grep -v grep | awk '{print $2}' |sed 's/\([0-9]*\)/-p \1/g' | tr "\n" " ")

strace -eclone $(ps -ef|grep gearmand | grep -v grep | awk '{print $2}' |sed 's/\([0-9]*\)/-p \1/g' | tr "\n" " ")


strace -c $(ps -ef|grep gearmand | grep -v grep | awk '{print $2}' |sed 's/\([0-9]*\)/-p \1/g' | tr "\n" " ")


 

brew install pidof
brew install dtruess
# vim ~/.bash_profile 	//mac
source /root/.bashrc
function straceall {
echo "sudo dtruss $(pidof "${1}" | sed 's/\([0-9]* \)/-p \1/g')"
sudo dtruss $(pidof "${1}" | sed 's/\([0-9]* \)/-p \1/g')
}

sed '/^ *#/d' **.conf > *.bak.conf



strace $(ps -ef|grep python | grep -v grep | grep app | awk '{print $2}' | sed 's/\([0-9]*\)/-p \1/g' |tr "\n" " ")
```

# 跟踪端口通信数据
```
#然后可以指定端口 或者 正则表达式
ngrep port 80
ngrep -q '^GET .* HTTP/1.[01]'
```


# 查询进程使用的文件
```
lsof -n -P | grep PID
```

### iptables
```
iptables -A INPUT -p tcp -s 218.247.181.51 -j ACCEPT

#删除规则
iptables -D INPUT 2 

```

## 判断出当前环境所使用的虚拟技术
```
wget http://people.redhat.com/~rjones/virt-what/files/virt-what-1.15.tar.gz
tar zxf virt-what-1.15.tar.gz
cd virt-what-1.15/
./configure && make && make install
virt-what
```

## MAC优化
```
当管理员要维护一些数据的时候可以暂时停止Spotlight服务程序，这样他不会干扰你的备份数据工作。
停止: sudo mdutil -i off / #停止Volume／上的Spotlight索引服务
运行: sudo mdutil -i on / #启动Volume／上的Spotlight索引服务
```

## golang
```
export GOROOT=/usr/lib/golang
export GOPATH=/usr/local/go
export PATH=$PATH:$GOROOT/bin:$GOPATH/bin
export GO111MODULE=on
```
