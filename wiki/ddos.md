## 简介
```
服务器的攻击分为四类，cc攻击、syn攻击、udp攻击、tcp洪水攻击。
```

## CC

```
CC攻击是DDoS(分布式拒绝服务)的一种，这种一种比DDOS流氓行为更具有技术含量的一种攻击方式，CC攻击完全模拟正常访问行为，没有虚假IP，也没有大的流量异常，但一样会造成您的服务器无法正常连接，一条ADSL的普通用户发起的CC攻击就可以干掉一台高性能的服务器。
服务器呗CC攻击时，会出现80端口关闭的现象，即出现丢包和高延迟的现象， 因为80端口被大量的垃圾数据堵塞导致正常的连接被中止。可以通过在CMD命令窗口输入命令 netstat -an 来查看，如果看到类似如下大量显示雷同的连接记录基本就可以被CC攻击了：

……
TCP 192.168.1.3:80 192.168.1.6:2205 SYN_RECEIVED 4
A.网站出现service unavailable提示

B.CPU占用率很高

C.网络连接状态：netstat –na,若观察到大量的ESTABLISHED的连接状态 单个IP高达几十条甚至上百条

D.外部无法打开网站,软重启后短期内恢复正常,几分钟后又无法访问。
```

- CC类攻击检测
```
第一条命令：
tcpdump -s0 -A -n -i any | grep -o -E '(GET|POST|HEAD) .*' 
正常的输出结果类似于这样
POST /ajax/validator.php HTTP/1.1
```

## 第二种类型：SYN类攻击

```
A.CPU占用很高
B.网络连接状态：netstat –na,若观察到大量的SYN_RECEIVED的连接状态
```

- SYN类攻击检测
```
netstat -na
显示所有活动的网络连接
netstat -an | grep :80 | sort
显示所有80端口的网络连接并排序。80端口为http端口
netstat -n -p | grep SYN_REC | wc -l
查看当前有多少活动的SYNC_REC连接，最好值小于5.
netstat -n -p | grep SYN_REC | sort -u
列出所有连接过的IP地址
netstat -n -p | grep SYN_REC | awk ‘{print $5}’ | awk -F: ‘{print $1}’
列出所有发送SYN_REC连接节点的IP地址
netstat -ntu | awk ‘{print $5}’ | cut -d: -f1 | sort | uniq -c | sort -n
使用netstat命令计算每个主机连接到本机的连接数
netstat -anp | grep ‘tcp|udp’ | awk ‘{print $5}’ | cut -d: -f1 | sort | uniq -c | sort -n
列出所有连接到本机的udp或者tcp连接的数量
netstat -ntu | grep ESTAB | awk ‘{print $5}’ | cut -d: -f1 | sort | uniq -c | sort -nr
检查ESTABLISHED 连接并且列出每个IP地址的连接数量
Netstat -plan|grep :80| awk {‘print $5’} | cut -d: -f1 | sort | uniq -c | sort -nk 1          列出所有连接到本机80端口的IP地址和其他连接数
```

- 利用netstat工具来检测查看SYN连接

```
netstat -n -p -t |wc -l
```

- 防范也主要从两方面入手，一是sysctl的自身的关于syn方面的配置，二是防火墙策略上。
```
sysctl -w net.ipv4.tcp_syncookies=1 
# tcp syncookie，默认关闭
sysctl -w net.ipv4.tcp_max_syn_backlog=1280 
# syn队列，默认1024，》 1280可能工作不稳定，需要修改内核源码参数
sysctl -w net.ipv4.tcp_synack_retries=2 
# syn-ack握手状态重试次数，默认5，遭受syn-flood攻击时改为1或2
sysctl -w net.ipv4.tcp_syn_retries=2 
# 外向syn握手重试次数，默认4
```

- sysctl -a|grep syn
```
fs.quota.syncs = 0
fs.xfs.inherit_sync = 1
fs.xfs.xfssyncd_centisecs = 3000
net.ipv4.tcp_max_syn_backlog = 128
net.ipv4.tcp_syn_retries = 6
net.ipv4.tcp_synack_retries = 5
net.ipv4.tcp_syncookies = 1
sysctl: reading key "net.ipv6.conf.all.stable_secret"
net.ipv6.conf.all.max_desync_factor = 600
sysctl: reading key "net.ipv6.conf.default.stable_secret"
net.ipv6.conf.default.max_desync_factor = 600
sysctl: reading key "net.ipv6.conf.eth0.stable_secret"
net.ipv6.conf.eth0.max_desync_factor = 600
sysctl: reading key "net.ipv6.conf.lo.stable_secret"
net.ipv6.conf.lo.max_desync_factor = 600
```

### 第三种类型：UDP类攻击

```
A.观察网卡状况 每秒接受大量的数据包
B.网络状态：netstat –na TCP信息正常
```

```
检测端口是否打开：nc -zuv ip 端口
服务器监听端口：nc -l -u ip 端口（可以发送和接受信息）
客户端检测端口：nc -u ip 端口（可以发送和接受信息）
查看监听的tup端口：ss -ant
查看监听的udp端口：ss -anu
查看所有协议端口：ss -ano
```

### 第四种类型：TCP洪水攻击

```
A.CPU占用很高
B.netstat –na,若观察到大量的ESTABLISHED的连接状态 单个IP高达几十条甚至上百条，属于正常。 查看TCP端口连接数.
```

```
查看网络连接总数
# netstat -an |wc -l

查看某个特定ip的连接数
# netstat -an |grep 8.8.8.8 |wc -l

查看连接数等待time_wait状态连接数
# netstat -an |grep TIME_WAIT|wc -l

查看建立稳定连接数量
# netstat -an |grep ESTABLISHED |wc -l

查看不同状态的连接数
# netstat -an | awk '/^tcp/ {++y[$NF]} END {for(w in y) print w, y[w]}'

查看每个ip跟服务器建立的连接数
# netstat -nat|awk '{print$5}'|awk -F : '{print$1}'|sort|uniq -c|sort -rn
（PS：正则解析：显示第5列，-F : 以：分割，显示列，sort 排序，uniq -c统计排序过程中的重复行，sort -rn 按纯数字进行逆序排序）

查看每个ip建立的ESTABLISHED/TIME_OUT状态的连接数
# netstat -nat|grep ESTABLISHED|awk '{print$5}'|awk -F : '{print$1}'|sort|uniq -c|sort -rn
```

# 防御TCP洪水攻击方法

```
通过调整tcp参数来防范DDOS攻击 sysctl -a | grep syn 看到：SYN相关的配置 net.ipv4.tcp_max_syn_backlog = 1024

net.ipv4.tcp_syncookies = 0

net.ipv4.tcp_synack_retries = 5

net.ipv4.tcp_syn_retries = 5

tcp_max_syn_backlog是SYN队列的长度，tcp_syncookies是一个开关，是否打开SYN Cookie 功能，该功能可以防止部分SYN攻击。tcp_synack_retries和tcp_syn_retries定义SYN 的重试次数。加大SYN队列长度可以容纳更多等待连接的网络连接数，打开SYN Cookie功能可以阻止部分 SYN攻击，降低重试次数也有一定效果。 调整上述设置的方法是：

增加SYN队列长度到2048：

sysctl -w net.ipv4.tcp_max_syn_backlog=2048
打开SYN COOKIE功能：

sysctl -w net.ipv4.tcp_syncookies=1
降低重试次数：

sysctl -w net.ipv4.tcp_synack_retries=3

sysctl -w net.ipv4.tcp_syn_retries=3
为了系统重启动时保持上述配置，可将上述命令加入到/etc/rc.d/rc.local文件中。

防止同步包洪水（Sync Flood）

# iptables -A FORWARD -p tcp --syn -m limit --limit 1/s -j ACCEPT
也有人写作

#iptables -A INPUT -p tcp --syn -m limit --limit 1/s -j ACCEPT

--limit 1/s 限制syn并发数每秒1次，可以根据自己的需要修改
防止各种端口扫描

# iptables -A FORWARD -p tcp --tcp-flags SYN,ACK,FIN,RST RST -m limit --limit 1/s -j ACCEPT
Ping洪水攻击（Ping of Death）

# iptables -A FORWARD -p icmp --icmp-type echo-request -m limit --limit 1/s -j ACCEPT
```





