###《记录我的代码片段》
====================

这辈子就与代码为伍了。

### 我的相关地址
- [我的博客](http://midoks.cachecha.com)
- [我的微博](http://weibo.com/u/1504761980)

### 目录解释
- common
	- 一些公共文件
- plugins
	- 插件及demo
- wiki
	- 一些便捷说明
- algorithm
	- 算法相关
- lnmp
	- lnmp环境搭建shell

### c10k && c10m
```
http://openresty.org/en/

http://dpdk.org/download

```

### 常用跟踪调试
```
yum install -y strace

# vim /root/.bashrc //添加以下内容
source /root/.bashrc
function straceall {
strace $(pidof "${1}" | sed 's/\([0-9]*\)/-p \1/g')
}
```