#!/bin/bash
PATH=/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin:~/bin


# curl -fsSL  https://raw.githubusercontent.com/midoks/mgo_web/master/scripts/install_dev.sh | sh


# Linux 手动安装
# wget https://go.dev/dl/go1.19.1.linux-amd64.tar.gz
# sudo tar -C /usr/local -xzf go1.19.1.linux-amd64.tar.gz
# sudo ln -s /usr/local/go/bin/* /usr/bin/

# systemctl status mgo_web

# 手动编译
# go build main.go -o mgo_web && mgo_web web 

if [ ! -d /usr/local/go ];then
	wget https://golang.google.cn/dl/go1.26.2.linux-amd64.tar.gz
	tar -xvf go1.26.2.linux-amd64.tar.gz
	mv go /usr/local/
fi


# Debug Now
export PATH=/usr/local/go:$PATH:/root/go/bin
export GOPATH=/root/go


TAGRT_DIR=/usr/local/mgo_web_dev
mkdir -p $TAGRT_DIR
cd $TAGRT_DIR

export GIT_COMMIT=$(git rev-parse HEAD)
export BUILD_TIME=$(date -u '+%Y-%m-%d %I:%M:%S %Z')


if [ ! -d $TAGRT_DIR/mgo_web ]; then
	git clone https://github.com/midoks/mgo_web
	cd $TAGRT_DIR/mgo_web
else
	cd $TAGRT_DIR/mgo_web
	git pull
fi

go mod tidy
go mod vendor

# cd /usr/local/mgo_web_dev/mgo_web && go build -o mgo_web main.go 
# cd /usr/local/mgo_web_dev/mgo_web && go build -o mgo_web main.go && ./mgo_web web
cd $TAGRT_DIR/mgo_web && go build -o mgo_web main.go 
systemctl daemon-reload


cd $TAGRT_DIR/mgo_web && ./mgo_web install
systemctl restart mgo_web

cd $TAGRT_DIR/mgo_web && ./mgo_web -v

systemctl status mgo_web



# systemctl status mgo_web
# journalctl -u mgo_web -f
# systemctl stop mgo_web
# systemctl restart mgo_web