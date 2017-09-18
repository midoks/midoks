### Linux安装问题

```
在虚拟机centos7上安装nginx之后虚拟机内能访问，真机不能访问，修改iptables配置也不起作用，最后上网查找了资料后才发现centos的防火墙改成了firewall,不再叫iptables,开放端口的方法如下


firewall-cmd --zone=public --add-port=80/tcp --permanent

systemctl stop firewalld.service  
systemctl start firewalld.service 
```



```
error while loading shared libraries: libopencv_core.so.2.4: cannot open shared object file: No such file or directory

在/usr/local/lib下面，在/etc/ld.so.conf.d/下面新建一个opencv.conf，里面写入/usr/local/lib，最后执行下sudo ldconfig -v即可。
```