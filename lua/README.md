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

### redis
```
local mdredis = require "resty.mdlib.mdredis"
local red = mdredis:new({ip = "127.0.0.1", port = 6379})

local ok, err = red:cmd('set', 'k', 'v', 10)
if nil ~= err then
	ngx.say("set fail")
end
local v = red:cmd('get', 'k')
ngx.say("v:",v)
```

### tc (测试)
```
local lib = require "resty.mdlib.mdtc"


-- method 1
lib:tc(function(param)
	ngx.say("message "..param)
	ngx.say("message "..nil)
end, "test trycall")
```