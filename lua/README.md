### Lua 简单学习

### openresty lua 库
```
写一些简单的方法放在mdlib中。

在mdlib放在../openresty/nginx/lualib/resty目录下,就可以测试下面的代码

-- test code
local regxlib = require "resty.mdlib.regxlib"
local rlib = regxlib:new()

local  isPhoneNumber = "14880283655"
ngx.say(isPhoneNumber,":","isPhoneNumber:",rlib:isPhoneNumber(isPhoneNumber))

local  mail = "midoks@163.com"
ngx.say(mail,":","isMail:",rlib:isMail(mail))

local url = "https://midoks.github.com/"
local b = rlib:isHttpUrl(url)
ngx.say(url,":",b)
```