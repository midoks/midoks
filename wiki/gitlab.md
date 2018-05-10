# GitLab

# 环境搭建
```
sudo yum install -y curl policycoreutils-python openssh-server
sudo systemctl enable sshd
sudo systemctl start sshd
sudo firewall-cmd --permanent --add-service=http
sudo systemctl reload firewalld



sudo yum install postfix
sudo systemctl enable postfix
sudo systemctl start postfix


curl https://packages.gitlab.com/install/repositories/gitlab/gitlab-ee/script.rpm.sh | sudo bash

sudo EXTERNAL_URL="http://gitlab.example.com" yum install -y gitlab-ee

sudo gitlab-ctl reconfigure

```

# Simply download one of the binaries for your system
```
# Linux x86-64
sudo wget -O /usr/local/bin/gitlab-runner https://gitlab-runner-downloads.s3.amazonaws.com/latest/binaries/gitlab-runner-linux-amd64

# Linux x86
sudo wget -O /usr/local/bin/gitlab-runner https://gitlab-runner-downloads.s3.amazonaws.com/latest/binaries/gitlab-runner-linux-386

# Linux arm
sudo wget -O /usr/local/bin/gitlab-runner https://gitlab-runner-downloads.s3.amazonaws.com/latest/binaries/gitlab-runner-linux-arm
```

# Give it permissions to execute
```
sudo chmod +x /usr/local/bin/gitlab-runner
```

# Optionally, if you want to use Docker, install Docker with
```
curl -sSL https://get.docker.com/ | sh
yum install docker.com
```

# Create a GitLab CI user
```
sudo useradd --comment 'GitLab Runner' --create-home gitlab-runner --shell /bin/bash
```

# Install and run as service
```
sudo gitlab-runner install --user=gitlab-runner --working-directory=/home/gitlab-runner
sudo gitlab-runner start
```


# Check
``` 
gitlab-ctl tail #查看所有日志
gitlab-ctl tail nginx/gitlab_access.log #查看nginx访问日志
```