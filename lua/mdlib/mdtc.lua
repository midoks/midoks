-- Copyright (C) by Midoks(midoks@163.com)

local setmetatable = setmetatable
local unpack = unpack
local cjson = require "cjson"

local _M = { _VERSION = '0.01' }
local mt = { __index = _M }

local function __TRACEBACK__(errmsg)
    local track_text = debug.traceback(tostring(errmsg), 6)
    ngx.say("---------------------------------------- TRACKBACK ----------------------------------------")
    ngx.say(track_text, "LUA ERROR")
    ngx.say("---------------------------------------- TRACKBACK ----------------------------------------")
    local exception_text = "LUA EXCEPTION\n" .. track_text
end

function _M.new(self)
    local p = setmetatable({
    }, mt)
    return p
end


function _M.tc( self, func , ... )
    local args = { ... }
    return xpcall(function() func(unpack(args)) end , __TRACEBACK__ )
end

function _M.test(self)  
    self.tc(self, function(param)
        ngx.say("message "..param)
        ngx.say("message "..nil)
    end, "test trycall")
end

return _M

