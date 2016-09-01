local cjson = require "cjson"

ngx.header.content_type="text/plain"

-- ngx.say("test2")

-- local regxlib = require "resty.mdlib.regxlib"
-- local rlib = regxlib:new()

-- local  isPhoneNumber = "14880283655"
-- ngx.say(isPhoneNumber,":","isPhoneNumber:",rlib:isPhoneNumber(isPhoneNumber))

-- local  mail = "midoks@163.com"
-- ngx.say(mail,":","isMail:",rlib:isMail(mail))

-- local url = "https://midoks.github.com/"
-- local b = rlib:isHttpUrl(url)
-- ngx.say(url,":",b)

ngx.say('1:',"2112:", ngx.status,":",ngx.header.referer)