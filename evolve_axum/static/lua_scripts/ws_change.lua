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

function Handler:join(input)
    local session_name = input.name or (redis.call("EXISTS", self:session_name_key(input.id)) == 1 and
        redis.call("GET", self:session_name_key(input.id))) or "undefined"
    redis.call("SET", self:session_name_key(input.id), session_name)
    redis.call("SADD", self:service_sessions_key(input.service_id), input.id)
    local time = redis.call("TIME")
    local now = time[1] * 1000 + math.floor(time[2] / 1000)
    redis.call("ZADD", self:session_rooms_key(input.id), now, input.room)
    redis.call("SADD", self:room_sessions_key(input.room), input.id)
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

function Handler:update_name(input)
    if (input.name and redis.call("EXISTS", self:session_name_key(input.id)) == 1) then
        redis.call("SET", self:session_name_key(input.id), input.name)
    end
end

function Handler:update_room_order(input)
    local time = redis.call("TIME")
    local now = time[1] * 1000 + math.floor(time[2] / 1000)
    redis.call("ZADD", self:session_rooms_key(input.id), now, input.room)
end

function Handler:handle()
    local input = json.decode(ARGV[1]);
    local output = {
        status = 0,
        msg = ""
    }
    if (input.type == "Join") then
        self:join(input)
    elseif (input.type == "Quit") then
        self:quit(input)
    elseif (input.type == "UpdateName") then
        self:update_name(input)
    elseif (input.type == "UpdateRoomOrder") then
        self:update_room_order(input)
    else
        output.status = 1;
        output.msg = string.format("wrong type: %q", input.type);
    end
    return json.encode(output)
end

return Handler:new():handle()
