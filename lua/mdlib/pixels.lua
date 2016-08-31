-- Copyright (C) by Midoks(midoks@163.com)

local cjson = require "cjson"

local setmetatable = setmetatable
local error = error


local _M = { _VERSION = '0.01' }
local mt = { __index = _M }

function _M.new(self)
    local p = setmetatable({

    }, mt)
    return p
end

function _M.one(self)
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
    return true
end


function _M.ones(self)
    ngx.header.content_type = "image/jpeg"
    ngx.say("GIF89a€˙˙˙˙,D;")
    return true
end

return _M

-- test
-- local ylib = require "resty.yokalib.pixels"
-- ylib:new()
-- ylib:one()