# coding:utf-8

import time
import sys
import re
from random import Random
from urllib.parse import unquote
import math
import random
import uuid
import shutil

import os

def encodeImage(imgsrc, newsrc):
    # 图片加密
    import struct
    old_fp = open(imgsrc, 'rb')
    imgFile = old_fp.read()
    old_fp.close()

    new_fp = open(newsrc,"wb")
    for x in imgFile:
        value = x ^ 136
        value = hex(value)
        s = struct.pack('B',int(value,16))
        new_fp.write(s)
    new_fp.close()
    return True


encodeImage("t1.webp","t1_xx.webp")