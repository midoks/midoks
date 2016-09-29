#! /usr/bin/python
# -*- coding: utf-8 -*-

import os,sys,time, atexit
from signal import SIGTERM

##http://www.cnblogs.com/gmark/archive/2012/09/27/2706339.html

class dd:
    def __init__(self):
        print "init"
        self.stdin = "/dev/null"
        self.stdout = "/Users/tangyihang/Desktop/python_cron/stdout.log"
        self.stderr = "/Users/tangyihang/Desktop/python_cron/error.log"
        self.pidfile = "/Users/tangyihang/Desktop/python_cron/id.pid"

    def create(self):
        print "create"

        #脱离父进程
        try:
            pid = os.fork()
            print pid
        except OSError, e:
            sys.stderr.write("fork #1 failed: %d (%s)\n" % (e.errno, e.strerror))
            sys.exit(1)


        #脱离终端
        os.setsid()

        #修改当前工作目录
        os.chdir("/")

        #重设文件创建权限
        os.umask(0)

        #第二次fork，禁止进程重新打开控制终端
        try:
            pid = os.fork()
            if pid > 0:
                sys.exit(0)
        except OSError, e:
            sys.stderr.write("fork #2 failed: %d (%s)\n" % (e.errno, e.strerror))
            sys.exit(1)


        sys.stdout.flush()
        sys.stderr.flush()
        si = file(self.stdin, 'r')
        so = file(self.stdout, 'a+')
        se = file(self.stderr, 'a+', 0)


        #重定向标准输入/输出/错误
        os.dup2(si.fileno(), sys.stdin.fileno())
        os.dup2(so.fileno(), sys.stdout.fileno())
        os.dup2(se.fileno(), sys.stderr.fileno())

        print "1231"


        print pid
        print self.pidfile
        f = open(self.pidfile,'w+')
        f.write("%s\n" % pid)
        f.close()

         #注册程序退出时的函数，即删掉pid文件
        atexit.register(self.delpid)
        pid = str(os.getpid())


    def delpid(self):
        os.remove(self.pidfile)

    def start(self):
        """
        Start the daemon
        """
        #print self.pidfile
        # Check for a pidfile to see if the daemon already runs
        try:
            pf = file(self.pidfile,'r')
            pid = int(pf.read().strip())
            pf.close()
        except IOError:
            pid = None

        if pid:
            message = "pidfile %s already exist. Daemon already running?\n"
            sys.stderr.write(message % self.pidfile)
            sys.exit(1)

        # Start the daemon
        self.create()


# test
if __name__ == "__main__":
    print __name__
    d = dd()
    d.start()
