### 使用说明

- 安装python3.7/3.8版本
bash <(curl -sSL https://raw.githubusercontent.com/midoks/choose-linux-python/main/install.sh)

/usr/local/python3.7.17/bin/pip3 install proxybroker
/usr/local/python3.8.17/bin/pip3 install proxybroker

- 加入计划任务
/usr/local/python3.7.17/bin/python3 /opt/proxybroker/gproxy.py
/usr/local/python3.8.17/bin/python3 /opt/proxybroker/gproxy.py

- 代理信息在此文件中
/tmp/tmp_proxy.json