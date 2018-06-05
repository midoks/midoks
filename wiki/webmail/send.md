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