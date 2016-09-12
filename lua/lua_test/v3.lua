
local redis = require "resty.redis"
local red = redis:new()
local cjson = require "cjson"

red:set_timeout(60000) -- 1min

local ok, err = red:connect("10.46.214.152", 6379)
if not ok then
	ngx.say(ERR,"failed to connect:", err)
end

local args = ngx.req.get_uri_args()
local j = {}
for i,v in pairs(args) do
    local key = 'l_'..i
    j[key] = v
end

j['host'] = ngx.var.host 
j['req_ip'] = ngx.var.remote_addr
j['repo'] = "v3"

local key = "logstash"
local message = cjson.encode(j)
red:rpush(key, message)
red:set_keepalive(100000, 1000)

function retjpg()
    local ylib = require "resty.yokalib.pixels"
    ylib:new()
    ylib:one()
end

-- 返回一像素图片
retjpg()