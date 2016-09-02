
local redis = require "resty.redis"
local red = redis:new()
local cjson = require "cjson"

red:set_timeout(1000) -- 1s

local ok, err = red:connect("127.0.0.1", 6379)

if not ok then
	ngx.say("failed to connect:", err)
end

local log_json = {}  
log_json['repo']="bbs"
log_json["uri"]=ngx.var.uri  
log_json["args"]=ngx.var.args  
log_json["host"]=ngx.var.host  
log_json["request_body"]=ngx.var.request_body  
log_json["remote_addr"] = ngx.var.remote_addr  
log_json["remote_user"] = ngx.var.remote_user  
log_json["time_local"] = ngx.var.time_local  
log_json["status"] = ngx.var.status  
log_json["body_bytes_sent"] = ngx.var.body_bytes_sent  
log_json["http_referer"] = ngx.var.http_referer  
log_json["http_user_agent"] = ngx.var.http_user_agent  
log_json["http_x_forwarded_for"] = ngx.var.http_x_forwarded_for  
log_json["upstream_response_time"] = ngx.var.upstream_response_time  
log_json["request_time"] = ngx.var.request_time
log_json["test"] = "123123"

local key = "logstash"
local message = cjson.encode(log_json)
red:lpush(key, message)


ngx.header.content_type = "text/plain";

ngx.say("1:","1")