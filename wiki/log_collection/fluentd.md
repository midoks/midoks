# 资料
- https://docs.fluentd.org/
- http://fluentular.herokuapp.com/

# mac安装
- https://td-agent-package-browser.herokuapp.com/2/macosx 

# 插件安装问题,gem可能连接不上.可以使用taobao的源代替官方的源.
```
sudo gem sources -l 
sudo gem sources -r http://rubygems.org
sudo gem sources -r https://rubygems.org
sudo gem sources -a https://ruby.taobao.org/
```

# 插件
```
- mac
/opt/td-agent/usr/sbin/td-agent-gem update
/opt/td-agent/usr/sbin/td-agent-gem install fluent-plugin-elasticsearch
/opt/td-agent/usr/sbin/td-agent-gem install fluent-plugin-typecast


- *inux
/usr/sbin/td-agent-gem update
/usr/sbin/td-agent-gem  install fluent-plugin-elasticsearch

```

```
- 安装后文件
/Library/LaunchDaemons/td-agent.plist
/etc/td-agent
/opt/td-agent
/var/log/td-agent


- deamon启动
sudo launchctl load /Library/LaunchDaemons/td-agent.plist

- deamon停止
sudo launchctl unload /Library/LaunchDaemons/td-agent.plist


```

# *inux
```
- 安装
curl -L https://toolbelt.treasuredata.com/sh/install-redhat-td-agent2.sh | sh

chkconfig --add td-agent
chkconfig td-agent on
/etc/init.d/td-agent start
``` 
