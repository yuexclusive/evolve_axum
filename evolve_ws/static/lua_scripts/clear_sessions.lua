--[[
Handler Object, for change the room and session
--]]
local Handler = {}

function Handler:new()
    local res = {}
    setmetatable(res, {
        __index = self
    })
    return res
end

function Handler:session_name_key(id)
    return id .. "_name"
end

function Handler:session_rooms_key(id)
    return id .. "_rooms"
end

function Handler:room_sessions_key(id)
    return id .. "_uids"
end

function Handler:service_sessions_key(service_id)
    return service_id .. "_service_uids"
end

function Handler:quit(input)
    redis.call("SREM", self:room_sessions_key(input.room), input.id)
    if redis.call("ZREM", self:session_rooms_key(input.id), input.room) == 1 then
        if redis.call("EXISTS", self:session_rooms_key(input.id)) == 0 then
            redis.call("DEL", self:session_name_key(input.id))
            redis.call("SREM", self:service_sessions_key(input.service_id), input.id)
        end
    end
end

function Handler:handle()
    local uids = redis.call("SMEMBERS", self:service_sessions_key(ARGV[1]))
    for _, uid in ipairs(uids) do
        local rooms = redis.call("ZRANGE", self:session_rooms_key(uid), 0, 1000)
        for _, room in ipairs(rooms) do
            self:quit({ id = uid, room = room, service_id = ARGV[1] })
        end
    end
    local output = {
        status = 0,
        msg = ""
    }
    return json.encode(output)
end

return Handler:new():handle()
