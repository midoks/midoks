
local retjpg = function()
    local list = {
        0x47,0x49,0x46,0x38,0x39,0x61,
        0x01,0x00,0x01,0x00,0x80,0xff,
        0x00,0xff,0xff,0xff,0x00,0x00,
        0x00,0x2c,0x00,0x00,0x00,0x00,
        0x01,0x00,0x01,0x00,0x00,0x02,
        0x02,0x44,0x01,0x00,0x3b
    }

    ngx.header.content_type = "image/jpeg"
    local s = ""
    for i=1,table.getn(list) do
        s = s..string.format("%c", list[i])
    end
    ngx.say(s)
end

local redis = require "resty.redis"
local red = redis:new()
local cjson = require "cjson"

red:set_timeout(60000) -- 1min

local ok, err = red:connect("127.0.0.1", 6379)
if not ok then
	ngx.say(ERR,"failed to connect:", err)
end

local args = ngx.req.get_uri_args()
local j = {}
for i,v in pairs(args) do
    local key = 'yk_'..i
    j[key] = v
end

j['host'] = ngx.var.host
j['req_ip'] = ngx.var.remote_addr
j['v'] = "v3"

local key = "logstash"
local message = cjson.encode(j)
red:rpush(key, message)
red:set_keepalive(100000, 1000)

ngx.header.content_type = "text/plain"
local u = ngx.req.get_uri_args()["u"]
if(u) then
	ngx.redirect(u)
else
	-- 返回一像素图片
	retjpg()
end


