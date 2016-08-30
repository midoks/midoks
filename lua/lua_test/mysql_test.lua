
local  mysql = require "resty.mysql"

local db,err = mysql:new()

if not db then
	ngx.say("failed to mysql", err)
	return
end

db:set_timeout(1000) -- 1 sec

local ok, err, errno, sqlstate = db:connect{
	host = "127.0.0.1",
	port = 3306,
	database = "test_1",
	user = "root",
	password = "root",
	max_packet_size = 1024 * 1024 
}

if not ok then
	ngx.say("fail to connect:", err, errno, sqlstate)
	return
end

ngx.say("connect to mysql<br/>")


res, err, errno, sqlstate = db:query("select * from t3 limit 1")
if not res then
	ngx.say("bad result:", err, ":", "errno", ":", sqlstate,".")
	return
end

-- ngx.say(res)

local cjson = require "cjson"
ngx.say("result: ", cjson.encode(res))

 if not res then
    ngx.say("bad result: ", err, ": ", errno, ": ", sqlstate, ".")
    return
end