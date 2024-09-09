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

## aria2c

```
aria2c --enable-rpc --rpc-allow-origin-all -c -D
```


## apt-listchanges --apt Segmentation fault
```

Segmentation fault
E: Sub-process /usr/bin/apt-listchanges --apt || test $? -lt 10 returned an error code (1)
E: Failure running script /usr/bin/apt-listchanges --apt || test $? -lt 10

修改/etc/apt/apt.conf.d/20listchanges文件,注释掉第一行.
#DPkg::Pre-Install-Pkgs { "/usr/bin/apt-listchanges --apt || test $? -lt 10"; };
```

## vpn

```
wget --no-check-certificate https://raw.githubusercontent.com/teddysun/across/master/l2tp.sh
chmod +x l2tp.sh
./l2tp.sh
```

## bbr

```
wget --no-check-certificate https://github.com/teddysun/across/raw/master/bbr.sh
chmod +x bbr.sh
./bbr.sh
```

## ssh代理

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
xattr -cr *
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

## LINUX如何查看哪个进程占用的SWAP分区比较多
```
for i in $(ls /proc | grep "^[0-9]" | awk '$0>100'); do awk '/Swap:/{a=a+$2}END{print '"$i"',a/1024"M"}' /proc/$i/smaps;done| sort -k2nr | head
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


## strace python
strace $(ps -ef|grep python | grep -v grep | grep app | awk '{print $2}' | sed 's/\([0-9]*\)/-p \1/g' |tr "\n" " ")
```


## 查看占用内存最高的5个进程

```
ps aux | sort -k4nr | head -n 5
```

## 查看占用cpu最高的5个进程

```
ps aux | sort -k3nr | head -n 5
```

## 跟踪端口通信数据
```
#然后可以指定端口 或者 正则表达式
ngrep port 80
ngrep -q '^GET .* HTTP/1.[01]'
```
## 服务流量统计

```
apt/yum install -y nethogs
nethogs -d 5
```

## PHP-FPM
```
1、查看php-fpm的进程个数
ps -ef |grep "php-fpm"|grep "pool"|wc -l

2、查看每个php-fpm占用的内存大小
ps -ylC php-fpm --sort:rss

3.查看PHP-FPM在你的机器上的平均内存占用
ps --no-headers -o "rss,cmd" -C php-fpm | awk '{ sum+=$1 } END { printf ("%d%s\n", sum/NR/1024,"M") }'

4.查看单个php-fpm进程消耗内存的明细
pmap $(pgrep php-fpm) | less

5.php-fpm的高CPU使用率排查方法
grep -v "^$" www.log.slow.tmp | cut -d " " -f 3,2 | sort | uniq -c | sort -k1,1nr | head -n 50
```

## 查询进程使用的文件
```
lsof -n -P | grep PID
```

## 正在使用cpu的进程

```
ps aux | awk '$3>0{print}'
```

### iptables
```
iptables -A INPUT -p tcp -s 218.247.181.51 -j ACCEPT

#删除规则
iptables -D INPUT 2 

```

### 磁盘读写测速
```
apt install -y hdparm

fdisk -l

hdparm -Tt /dev/sda5
hdparm -Tt /dev/sda1

hdparm -Tt /dev/nvme0n1p1
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

## MAC修复，文件已经损坏的提示
```
sudo xattr -r -d com.apple.quarantine /xx.xx
```

## sqlite修复
```
error: database disk image is malformed

sqlite3 test.db ".dump" >> back.sql
cat ./back.sql.sql | grep -v TRANSACTION | grep -v ROLLBACK | grep -v COMMIT >./new_back.sql
#sqlite3 test.db ".dump" | grep -v TRANSACTION | grep -v ROLLBACK | grep -v COMMIT  | grep -v "ERROR" >> new_back.sql
sqlite3 test.db < new_back.sql
```

## 查看端口占用
```
netstat -ntulp
```

## 临时设置/销毁Git代理

```
git config --global http.proxy 'socks5://127.0.0.1:1082'
git config --global https.proxy 'socks5://127.0.0.1:1082'

git config --global --unset http.proxy
git config --global --unset https.proxy
```

### pip
```
pip install urllib3==1.23 -i https://pypi.tuna.tsinghua.edu.cn/simple
```

## golang
```
export GOROOT=/usr/lib/golang
export GOPATH=/usr/local/go
export PATH=$PATH:$GOROOT/bin:$GOPATH/bin
export GO111MODULE=on
```
