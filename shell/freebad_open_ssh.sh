#!/bin/bash


# curl -fsSL  https://raw.githubusercontent.com/midoks/midoks/master/shell/freebad_open_ssh.sh | bash
# freebsd 开启ssh登录



sed -i '' 's/^#PermitRootLogin no/PermitRootLogin yes/g' /etc/ssh/sshd_config
sed -i '' 's/^#PasswordAuthentication no/PasswordAuthentication yes/g' /etc/ssh/sshd_config
sed -i '' 's/^#Port 22/Port 2022/g' /etc/ssh/sshd_config

# cat  /etc/ssh/sshd_config | grep PermitRootLogin
# cat  /etc/ssh/sshd_config | grep PasswordAuthentication
# cat  /etc/ssh/sshd_config | grep Port
/etc/rc.d/sshd restart
# /etc/netstart restart
