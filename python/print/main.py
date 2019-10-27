#!/usr/bin/python
# -*- coding: UTF-8 -*-

# https://docs.python.org/3/library/tk.html

# pip install python-escpos
# pip install escpos
# pip install Tkinter


import Tkinter
import tkMessageBox

from escpos import *


top = Tkinter.Tk()

# 窗口设置
top.title("查询")
# top.geometry("500x300")
# 设置窗口大小
width = 600
height = 300
# 获取屏幕尺寸以计算布局参数，使窗口居屏幕中央
screenwidth = top.winfo_screenwidth()
screenheight = top.winfo_screenheight()
alignstr = '%dx%d+%d+%d' % (width, height,
                            (screenwidth - width) / 2, (screenheight - height) / 2)
top.geometry(alignstr)
top.resizable(width=False, height=False)


def printerVV():
    usb = printer.Usb(0x0fe6, 0x811e, 0, out_ep=0x03)
    usb.text(u"终于可以愉快的打印啦\n\n\n\n\n\n\n\n".encode('gbk'))
    usb.qr("值")  # 打印二维码
    usb.set(codepage=None, align="center")  # 设置页面居中
    usb.cut()  # 切纸
    usb.close()  # 关闭连接


def printValue():
    printerVV()
    tkMessageBox.showinfo("Hello Python", "printerVV")

B = Tkinter.Button(top, text="打印", command=printValue)
B.pack()


inputQuery = Tkinter.Entry(top)
inputQuery.pack()


def findValue():
    v = inputQuery.get()
    tkMessageBox.showinfo("查询值", v)

B = Tkinter.Button(top, text="查询", command=findValue)
B.pack()

top.mainloop()
