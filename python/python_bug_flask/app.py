# coding=utf-8

# ---------------------------------------------------------------------------------
# MW-Linux面板
# ---------------------------------------------------------------------------------
# copyright (c) 2018-∞(https://github.com/midoks/mdserver-web) All rights reserved.
# ---------------------------------------------------------------------------------
# Author: midoks <midoks@163.com>
# ---------------------------------------------------------------------------------



import sys
import os

from admin import app

# 我们需要在sys.path中包含根目录，以确保我们可以找到在独立运行时运行时所需的一切。
if sys.path[0] != os.path.dirname(os.path.realpath(__file__)):
    sys.path.insert(0, os.path.dirname(os.path.realpath(__file__)))


def main():
    app.run()
 

if __name__ == '__main__':
    import eventlet
    eventlet.monkey_patch()
    main()
