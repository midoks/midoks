#!/bin/sh

#变成可执行文件
chmod +x ./before_install.sh
chmod +x ./mysql.sh
chmod +x ./php.sh
chmod +x ./nginx.sh
chmod +x ./after_install.sh

#执行
sh ./before_install.sh

if [ $? -nq 0 ]; then
	echo "failed before_install.sh"
	exit 1
fi

sh ./mysql.sh

if [ $? -nq 0 ]; then
	echo "failed mysql.sh"
	exit 1
fi

sh ./php.sh

if [ $? -nq 0 ]; then
	echo "failed php.sh"
	exit 1
fi

sh ./nginx.sh

if [ $? -nq 0 ]; then
	echo "failed nginx.sh"
	exit 1
fi

sh ./after_install.sh

if [ $? -nq 0 ]; then
	echo "failed after_install.sh"
	exit 1
fi
