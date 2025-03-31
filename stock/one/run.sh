#!/bin/bash
PATH=/usr/local/bin:/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin:~/bin

if [ -f bin/activate ];then
    source bin/activate
fi

BS_VERSION(){
    echo "0.0.30"
}

BS_VENV(){
    python3 -m venv .
    source bin/activate && pip install baostock
}

BS_HELP(){
    echo "./run.sh run|r            -> 策略1"
    echo "./run.sh r2               -> 下载最新数据并执行策略"
    echo "./run.sh venv             -> 创建venv环境"
    echo "./run.sh download|d       -> 下载数据"
    echo "./run.sh version|v        -> 版本信息"
}


BS_DOWNLOAD(){
    python3 download.py
}

BS_CMD(){
    python3 analysis.py
}

BS_RUNTIME(){
    rm -rf ./code/*.csv
    python3 download.py
    python3 analysis.py
}


case "$1" in
    "run" | "r") BS_CMD ;;
    "r2") BS_RUNTIME ;;
    "download"| "d"  ) BS_DOWNLOAD ;;
    "version" | "v") BS_VERSION;;
    "venv") BS_VENV;;
    *) BS_HELP;;
esac