#!/bin/bash
PATH=/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin:~/bin:/usr/local/lib/python2.7/bin:/opt/homebrew/bin
curPath=`pwd`
rootPath=$(dirname "$curPath")
echo "web pnpm build start"

# exit 0
echo $curPath
echo $rootPath

# if [ -f $curPath/fastcdn-web/apps/web-naive/dist.zip ] && [ -f $curPath/fastcdn/public/dist.zip ];
# then
# 	web_md5=`md5sum $curPath/fastcdn-web/apps/web-naive/dist.zip | awk '{print $1}'`
# 	fastcdn_public_md5=`md5sum $curPath/fastcdn/public/dist.zip | awk '{print $1}'`

# 	echo "fastcdn:$fastcdn_public_md5"
# 	echo "webdev:$web_md5"
# 	if [ "$web_md5" == "$fastcdn_public_md5" ];then
# 		# rm -rf ${curPath}/fastcdn-web/apps/web-naive/dist.zip
# 		echo "web file no change!"
# 		exit 0
# 	fi
# fi

rm -rf ${curPath}/fastcdn/public/index.html
rm -rf ${curPath}/fastcdn/public/static

cd ${rootPath}/fastcdn-web && pnpm build
rm -rf ${curPath}/fastcdn/public/dist.zip
cp -rf ${rootPath}/fastcdn-web/apps/web-naive/dist.zip ${curPath}/fastcdn/public/dist.zip

cd ${curPath}/fastcdn/public
unzip -o dist.zip -d ./ 
echo "web cover end"

echo "rm -rf ${rootPath}/fastcdn-web/apps/web-naive/dist.zip"
rm -rf ${rootPath}/fastcdn-web/apps/web-naive/dist.zip
rm -rf ${curPath}/fastcdn/public/dist.zip

echo "web pnpm build end"
