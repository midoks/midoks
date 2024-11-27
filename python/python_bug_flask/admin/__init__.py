# coding:utf-8

# ---------------------------------------------------------------------------------
# MW-Linux面板
# ---------------------------------------------------------------------------------
# copyright (c) 2018-∞(https://github.com/midoks/mdserver-web) All rights reserved.
# ---------------------------------------------------------------------------------
# Author: midoks <midoks@163.com>
# ---------------------------------------------------------------------------------
import eventlet
eventlet.monkey_patch()

import os
import sys
import json
import time
import uuid
import logging

from datetime import timedelta


from flask import Flask
from flask import Blueprint, render_template

app = Flask(__name__, template_folder='templates/default')

@app.route('/')
def index():
    return "123"