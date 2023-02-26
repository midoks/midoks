## rclone安装使用

- 相关脚本
```

curl https://rclone.org/install.sh | sudo bash

yum install fuse -y
apt install fuse -y

#挂载
rclone mount gdrive: /gdrive --allow-other --allow-non-empty --vfs-cache-mode writes --daemon

#取消挂载
fusermount -qzu <本地路径>
```

# 验证

- google 验证的时候,需要通过google页面验证。
- 最好开两个cmd.一个配置，一个接收code,如下。

```
2023/01/03 08:45:09 NOTICE: Got code
Paste the following into your remote machine --->
xxx
<---End paste
```


```
ssh -L localhost:53682:localhost:53682 root@xx.xx.xx.xx 
```

# 一件安装

```
curl -fsSL https://raw.githubusercontent.com/midoks/midoks/master/shell/rclone_install.sh | bash
```



