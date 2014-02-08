#一般不变变的
PREFIX = /usr/local
CC = gcc	-Wall -ggdb -W -O
RM = -rm -rf

#变量
OBJECTS = wst.c socket.c
#命令执行
all: wst install .PHONY

#命令安装
#文档写入
install: wst
	install -s wst $(PREFIX)/bin#命令安装
	install -m 644 doc/man $(PREFIX)/man/man1
	install -d $(PREFIX)/share/doc/wst

wst : wst.o
	$(CC) -o wst wst.o

wst.o: $(OBJECTS)

#删除安装在系统的命令和其他文
del:
	rm -rf $(PREFIX)/bin/wst

.PHONY:	clean
clean:
	$(RM) *.o 
	$(RM) wst
