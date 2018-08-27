# email 发送过程

# send

```
c:telnet smtp.163.com 25
c:HELO mail
s:250 OK


c:AUTH LOGIN
s:334 dXNlcm5hbWU6
c:bWlkb2tzQDE2My5jb20=
s:334 UGFzc3dvcmQ6
c:MTQ5MDEycQ==
s:235 Authentication successful


c:MAIL FROM: <midoks@163.com>
s:250 Mail OK
c:RCPT TO: <chenjiangshan@g7.com.cn>
s:250 Mail OK
c:DATA
c:From:76126128@qq.com
c:To:8454051@qq.com
c:Subject:test
c:test
s:354 Enter mail, end with "." on a line by itself  
c:Enjoy Protocol Studing
c:.
c:QUIT
s:221 Bye


```

# send simple example
```


telnet smtp.163.com 25
HELO mail

AUTH LOGIN
bWlkb2tzQDE2My5jb20=
MTQ5MDEycQ==

MAIL FROM: <midoks@163.com>
RCPT TO: <chenjiangshan@g7.com.cn>
DATA
From:midoks@163.com
To:<chenjiangshan@g7.com.cn>
Subject:test
test
.
QUIT
```


## telnet 163mx03.mxmail.netease.com 25
```
midoksdeMacBook-Pro:~ midoks$ telnet 163mx03.mxmail.netease.com 25
Trying 220.181.14.161...
Connected to 163mx03.mxmail.netease.com.
Escape character is '^]'.
220 163.com Anti-spam GT for Coremail System (163com[20141201])
EHLO mail
250-mail
250-PIPELINING
250-AUTH LOGIN PLAIN 
250-AUTH=LOGIN PLAIN
250-coremail 1Uxr2xKj7kG0xkI17xGrU7I0s8FY2U3Uj8Cz28x1UUUUU7Ic2I0Y2UrIVpNiUCa0xDrUUUUj
250-STARTTLS
250-SIZE 73400320
250 8BITMIME
HELO mail
250 OK
MAIL FROM: <midoks@extmail.org>
250 Mail OK
RCPT TO: <midoks@163.com>
250 Mail OK
DATA
354 End data with <CR><LF>.<CR><LF>
From:midoks@test.com
To:<midoks@163.com>
Subject:test

test
.
QUIT250 Mail OK queued as mx3,NcCowACXgEvdnYNbYbVoOg--.25780S3 1535352352

221 Bye
Connection closed by foreign host.
```