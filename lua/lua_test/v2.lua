function retjpg()
    local ylib = require "resty.yokalib.pixels"
    ylib:new()
    ylib:one()
end

-- 返回一像素图片
-- retjpg()


local function send_redis_queue(msg)
    local cjson = require "cjson"
    
    local mdredis = require "resty.yokalib.mdredis"
    local red = mdredis:new({ip = "10.46.214.152", port = 6379})
    
    local ok, err = red:cmd('lpush', 'logstash', cjson.encode(msg))
    if nil ~= err then
        ngx.log(ngx.ERR, "push fail")
    end
    return true
end

local args = ngx.req.get_uri_args()
local j = {}
for i,v in pairs(args) do
    local key = 'l_'..i
    j[key] = v
end

j['host'] = ngx.var.host 
j['req_ip'] = ngx.var.remote_addr

send_redis_queue(j)