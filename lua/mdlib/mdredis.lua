-- Copyright (C) by Midoks(midoks@163.com)

local cjson = require "cjson"
local redis = require "resty.redis"
-- local red = redis:new()
local setmetatable = setmetatable
local unpack = unpack

local _M = { _VERSION = '0.01' }
local mt = { __index = _M }

function _M.new(self, opt)

    local  ip       = opt.ip        or "127.0.0.1"
    local  port     = opt.port      or 6379
    local  timeout  = opt.timeout   or 1000     -- 1s
    local  idletime = opt.idletime  or 10000    -- 10s
    local  poolsize = opt.poolsize  or 100 

    -- ngx.say(cjson.encode(opt))
    local p = setmetatable({
        ip       = ip,      
        port     = port,
        timeout  = timeout,
        idletime = idletime,    
        poolsize = poolsize
    }, mt)
    return p
end


local function _connect( self )
    local red = redis:new()

    red:set_timeout(self.timeout)

    local  ok, err = red:connect(self.ip, self.port)
    if not ok then
        return nil, err
    end

    return red, nil
end

local function _set_keepalive( self, red )
    red:set_keepalive(self.idletime, self.poolsize)
end


function _M.cmd(self, command, ...)
    local res, err  = nil, nil
    local select    = select
    local table     = table

    red, err  = _connect(self)
    if err then 
        return nil, err
    end

    -- 对set,特殊处理一下
    if 'set' == command and table.getn({...}) > 2 then
        local key       = select(1, ...)
        local value     = select(2, ...)
        local timeout   = select(3, ...)

        local cmd_func = red[command]
        res, err = cmd_func(red, key, value)
        red:expire(key, timeout)
    else 
        local cmd_func = red[command]
        res, err = cmd_func(red, ...)
    end

    if nil == err then 
        _set_keepalive(self, red)
    end

    return res, err
end


return _M

