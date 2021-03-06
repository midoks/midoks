function retjpg2()
	local ylib = require "resty.yokalib.pixels"
	ylib:new()
	ylib:one()
end

-- 返回一像素图片
retjpg2()


function send_kakfa_data(message, topic)

    local cjson = require "cjson"
    local client = require "resty.kafka.client"
    local producer = require "resty.kafka.producer"

    local broker_list = {
        { host = "10.46.214.164", port = 9092 },
        -- { host = "192.168.62.132", port = 9092 },
    }

    local key = 'key'
    local message = cjson.encode(message)

    -- usually we do not use this library directly
    local cli = client:new(broker_list)
    local brokers, partitions = cli:fetch_metadata(topic)
    if not brokers then
        -- ngx.say("fetch_metadata failed, err:", partitions)
        return false
    end

    local bp = producer:new(broker_list, { producer_type = "async" })
    local ok, err = bp:send(topic, key, message)
    if not ok then
        -- ngx.say("send err:", err)
        return false
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

send_kakfa_data(j, 'test')