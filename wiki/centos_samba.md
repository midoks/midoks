### samba搭建
```

#安装
yum install samba

#启动
server smb start|stop|status

##加入自启动
chkconfig smb on


#小技巧，只显示对我们有用的配置选项
grep -v "^[[:space:]]*#"  smb.conf  |  grep -v "^$" | grep -v "^:"


testparm smb.conf #使用testparm检查是否有语法错误


#smbpasswd命令
smbpasswd命令用于维护Samba服务器的用户帐号
添加Samba用户帐号
# smbpasswd -a sambauser 
禁用Samba用户帐号 
# smbpasswd -d sambauser
启用Samba用户帐号 
# smbpasswd -e sambauser
删除Samba用户帐号 
# smbpasswd -x sambauser

```


