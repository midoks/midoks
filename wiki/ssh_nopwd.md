### SSH免登陆设置

有机器A(192.168.1.155)，B(192.168.1.181)。现想A通过ssh免密码登录到B。

### 在A机下生成公钥/私钥对。
- ssh-keygen -t rsa
- ssh-keygen -t rsa -P '密码'

### 传到B中
scp id_rsa.pub root@120.76.123.86:/root/.ssh

### 在B中.ssh/
- cat id_rsa.pub >> authorized_keys 
- chmod 600 authorized_keys


### OK
