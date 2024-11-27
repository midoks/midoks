# coding:utf-8

# ---------------------------------------------------------------------------------
# MW-Linux面板
# ---------------------------------------------------------------------------------
# copyright (c) 2018-∞(https://github.com/midoks/mdserver-web) All rights reserved.
# ---------------------------------------------------------------------------------
# Author: midoks <midoks@163.com>
# ---------------------------------------------------------------------------------

# ---------------------------------------------------------------------------------
# 配置文件
# ---------------------------------------------------------------------------------


import time
import sys
import random
import os


threads = 1 * 1
backlog = 512
reload = True
daemon = False
# # worker_class = 'geventwebsocket.gunicorn.workers.GeventWebSocketWorker'
worker_class = 'eventlet'
timeout = 600
keepalive = 60
preload_app = True
capture_output = False
access_log_format = '%(t)s %(p)s %(h)s "%(r)s" %(s)s %(L)s %(b)s %(f)s" "%(a)s"'
loglevel = 'info'
errorlog = './logs/panel_error.log'
accesslog =  './logs/panel.log'
pidfile =  './logs/panel.pid'
