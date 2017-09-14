### Linux安装问题

```
在虚拟机centos7上安装nginx之后虚拟机内能访问，真机不能访问，修改iptables配置也不起作用，最后上网查找了资料后才发现centos的防火墙改成了firewall,不再叫iptables,开放端口的方法如下


firewall-cmd --zone=public --add-port=80/tcp --permanent

systemctl stop firewalld.service  
systemctl start firewalld.service 
```