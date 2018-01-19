### Docker 安装
- http://www.docker.com/

### 集群相关
```

```

# Linux安装
```
sudo yum install -y yum-utils device-mapper-persistent-data lvm2

sudo yum-config-manager --add-repo http://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo

sudo yum -y makecache fast
sudo yum -y install docker-ce
sudo service docker start
```

# docker 常用命令
```
docker info
docker version
docker-compose --version
docker-machine --version

docker run -d -p 80:80 --name webserver nginx
docker ps


docker start web 			#启动容器名为web的容器
docker exec -it web bash 	#进入容器
docker stop web 			#停止容器
docker rm web 				#删除容器


```

# 镜像的操作
```
docker images
docker rmi ad673a791d21  #删除images
```

# linux 
```



```


# mac
- https://docs.docker.com/docker-for-mac/
```


```
