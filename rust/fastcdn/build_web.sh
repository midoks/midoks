#!/bin/bash
PATH=/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin:~/bin:/usr/local/lib/python2.7/bin:/opt/homebrew/bin
curPath=`pwd`
echo "web pnpm build start"

cd fastcdn-web && pnpm build

cp -f apps/web-naive/dist.zip ../fastcdn/public/dist.zip

cd ../fastcdn/public

unzip -f -d ./ dist.zip


echo "web pnpm build end"
