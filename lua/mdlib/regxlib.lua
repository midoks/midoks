-- Copyright (C) by Midoks(midoks@163.com)

local cjson = require "cjson"

-- C语言链接的
-- local ffi = require "ffi"
-- local ffi_new = ffi.new
-- local ffi_str = ffi.string
-- local C = ffi.C
local setmetatable = setmetatable
local error = error


local _M = { _VERSION = '0.01' }
local mt = { __index = _M }

function _M.new(self)
    local p = setmetatable({

    }, mt)
    return p
end

function _M.test()
    return "test"
end

-- 判段是否是手机号
function _M.isPhoneNumber(self, phoneNumber)
    if string.len(phoneNumber) ~= 11 then return false end
    
    local regex = [[(1[3-8]\d+)+]]
    local m = ngx.re.match(phoneNumber, regex, "o")

    if m then 
        local mVal = m[0]
        local mLen = table.getn(m)

        if mLen ~= 1 then
            return true
        end

        if string.len(mVal) ~= 11 then
            return false
        end

        return true
    else 
        return false
    end
end


-- 判断是否是邮件地址
function _M.isMail(self, mail)

    local regex = [[^[\w][\w\._-]*@[\w\._-]*.[\w\._-]*$]]
    local m = ngx.re.match(mail, regex, "o")
    if m then 
        -- ngx.say('json:', ":", cjson.encode(m), table.getn(m));
        return true
    end
    return false
end





return _M

