#!/bin/bash
PATH=/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin:~/bin:/usr/local/lib/python2.7/bin:/opt/homebrew/bin
curPath=`pwd`


# fastcdn
mkdir -p dist
mkdir -p dist/fastcdn/logs
mkdir -p dist/fastcdn/configs
mkdir -p dist/fastcdn/bin
mkdir -p dist/fastcdn/web

mkdir -p dist/fastcdn/fastcdn-api
mkdir -p dist/fastcdn/fastcdn-api/configs
mkdir -p dist/fastcdn/fastcdn-api/bin
mkdir -p dist/fastcdn/fastcdn-api/deploy
mkdir -p dist/fastcdn/fastcdn-api/logs
mkdir -p dist/fastcdn/fastcdn-api/data


rm -rf ./dist/fastcdn/bin/fastcdn
cp -rf ./target/release/fastcdn ./dist/fastcdn/bin/fastcdn
cp -rf ./configs/server.yaml ./dist/fastcdn/configs/server.yaml


rm -rf ./dist/fastcdn/fastcdn-api/bin/fastcdn-api
cp -rfp ./target/release/fastcdn-api ./dist/fastcdn/fastcdn-api/bin/fastcdn-api


# fastcdn-node
mkdir -p dist/fastcdn-node/bin
mkdir -p dist/fastcdn-node/configs


rm -rf ./dist/fastcdn-node/bin/fastcdn-node
cp -rf ./target/release/fastcdn-node ./dist/fastcdn-node/bin/fastcdn-node