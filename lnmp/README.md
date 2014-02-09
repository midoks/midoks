《LNMP环境安装》
====================

这辈子就与代码为伍了。

### 我的相关地址
- [我的博客](http://midoks.cachecha.com)
- [我的微博](http://weibo.com/u/1504761980)


### 文件解释
- sysctl.conf
	- 内核优化参数
		- note:如果你使用VPS的时,也许并不能直接使用。你要使用一下命令进行修复
		- rm -f /sbin/modprobe
		- ln -s /bin/true /sbin/modprobe
		- rm -f /sbin/sysctl
		- ln -s /bin/true /sbin/sysctl
		- sysctl  -p (使你设置生效)
