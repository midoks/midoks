#!/bin/bash

# curl -x "socks://127.0.0.1:1069" google.com
# curl -x "127.0.0.1:10000" google.com
source ~/.zshrc  && proxy_on

# 开启代理
function proxy_on(){
    # port 根据你的代理工具自行设置
    export ALL_PROXY=socks://127.0.0.1:1069
    export http_proxy=http://127.0.0.1:10000
    # export https_proxy=https://127.0.0.1:10000
    echo -e "已开启代理"
}
# 关闭代理
function proxy_off(){
    unset ALL_PROXY
    unset http_proxy
    unset https_proxy
    echo -e "已关闭代理"
}
