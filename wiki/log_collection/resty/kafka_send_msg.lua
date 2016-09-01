
function filter_req()
	local args = ngx.req.get_headers()["referer"]
	local req = {
	}
	return true
end


function retjpg2()
	local ylib = require "resty.yokalib.pixels"
	ylib:new()
	ylib:one()
end

function send_kakfa_data2(message, topic)

	local cjson = require "cjson"
	local client = require "resty.kafka.client"
	local producer = require "resty.kafka.producer"

	local broker_list = {
	    { host = "10.46.214.164", port = 9092 },
	    -- { host = "192.168.62.132", port = 9092 },
	}

	local key = "key"
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

-- init_send_data()


-- test
-- local cjson = require "cjson"
-- local args = ngx.req.get_uri_args()
-- ngx.say(cjson.encode(args))


-- local v = {
-- 	repo 	= 'view',
-- 	purl 	= ngx.req.get_uri_args()['url'] or "",
-- 	ptitle 	= ngx.req.get_uri_args()['action_name'] or "",
-- 	cookie 	= ngx.req.get_uri_args()['cookie'] or "0",
-- 	java 	= ngx.req.get_uri_args()['java'] or "0",
-- 	realp 	= ngx.req.get_uri_args()['realp'] or "0",
-- 	wma 	= ngx.req.get_uri_args()['wma'] or "0",
-- 	dir 	= ngx.req.get_uri_args()['dir'] or "0",
-- 	gears 	= ngx.req.get_uri_args()['gears'] or "0",
-- 	res 	= ngx.req.get_uri_args()['res'] or "0",
-- 	pdf 	= ngx.req.get_uri_args()['pdf'] or "0",
-- 	send_image = ngx.req.get_uri_args()['send_image'] or "0",
-- 	cs 		= ngx.req.get_uri_args()['cs'] or "0",
-- 	idsite 	= ngx.req.get_uri_args()['idsite'] or "0",
-- 	rec 	= ngx.req.get_uri_args()['rec'] or "0",
-- 	r 		= ngx.req.get_uri_args()['r'] or "0",
-- 	h 		= ngx.req.get_uri_args()['h'] or "0",
-- 	m 		= ngx.req.get_uri_args()['m'] or "0",
-- 	s 		= ngx.req.get_uri_args()['s'] or "0",
-- 	s 		= ngx.req.get_uri_args()['s'] or "0",
-- 	p_id 	= ngx.req.get_uri_args()['_id'] or "0",
-- 	p_idts 	= ngx.req.get_uri_args()['_idts'] or "0",
-- 	p_idvc 	= ngx.req.get_uri_args()['_idvc'] or "0",
-- }

-- local cjson = require "cjson"
-- ngx.say(cjson.encode(v))

local args = ngx.req.get_uri_args()
local j = {}
for i,v in pairs(args) do
	local key = "l_"..i
	j[key] = v
end 
-- ngx.say(cjson.encode(j))
send_kakfa_data2(j, 'test')


-- 返回一像素图片
-- retjpg2()