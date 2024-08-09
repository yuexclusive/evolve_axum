use crate::error::AppError;
use crate::{dao::redis::lua_script, error::AppResult, redis_util};

use super::chat::DEFAULT_ROOM;
use super::memory_hub::Hub;
use redis::ConnectionLike;
use serde::{Deserialize, Serialize};

use redis::Commands;
use redis::FromRedisValue;
use redis_encoding_derive::{from_redis, to_redis};
use std::collections::HashSet;

const USER_NAME_POSTFIX: &str = "_name";
const USER_ROOM_POSTFIX: &str = "_rooms";
const ROOM_USER_POSTFIX: &str = "_uids";
pub struct RedisHub;

#[from_redis]
// #[to_redis]
pub enum RoomChangeType {
    Join,
    Quit,
    UpdateName,
}

#[to_redis]
#[from_redis]
pub struct ChangeRoomReq {
    pub id: String,
    pub name: Option<String>,
    pub room: String,
    pub r#type: RoomChangeType,
}

#[from_redis]
pub struct RoomChangeForHubResponse {
    pub status: i8,
    pub msg: String,
}

impl Hub for RedisHub {
    fn uname(&self, uid: &str) -> AppResult<String> {
        let res = redis_util::sync::conn()?
            .get::<_, String>(format!("{}{}", uid, USER_NAME_POSTFIX))
            .unwrap_or_default();

        Ok(res)
    }

    fn rooms(&self, uid: &str) -> AppResult<HashSet<String>> {
        let res = redis_util::sync::conn()?.smembers(format!("{}{}", uid, USER_ROOM_POSTFIX))?;
        Ok(res)
    }

    fn uids(&self, room: &str) -> AppResult<HashSet<String>> {
        let res = redis_util::sync::conn()?.smembers(format!("{}{}", room, ROOM_USER_POSTFIX))?;
        Ok(res)
    }

    fn is_already_in_room(&self, uid: &str, room: &str) -> AppResult<bool> {
        let res =
            redis_util::sync::conn()?.sismember(uid, format!("{}{}", room, ROOM_USER_POSTFIX))?;
        Ok(res)
    }

    fn join(&self, uid: &str, uname: &str, room: &str) -> AppResult<()> {
        let mut cmd = redis::cmd("evalsha");

        cmd.arg(lua_script::get_rooms_change_sha()?) //sha
            .arg(0) //keys number
            .arg(&ChangeRoomReq {
                id: uid.to_string(),
                name: Some(uname.to_string()),
                room: room.to_string(),
                r#type: RoomChangeType::Join,
            });

        let value = redis_util::sync::conn()?.req_command(&cmd)?;

        let res = RoomChangeForHubResponse::from_redis_value(&value)?;

        if res.status != 0 {
            return Err(AppError::Internal { msg: res.msg });
        }

        Ok(())
    }

    fn quit(&self, uid: &str, room: Option<&str>) -> AppResult<()> {
        let mut cmd = redis::cmd("evalsha");

        match room {
            Some(room) => {
                cmd.arg(lua_script::get_rooms_change_sha()?) //sha
                    .arg(0) //keys number
                    .arg(&ChangeRoomReq {
                        id: uid.to_string(),
                        name: None,
                        room: room.to_string(),
                        r#type: RoomChangeType::Quit,
                    });

                let value = redis_util::sync::conn()?.req_command(&cmd)?;

                let res = RoomChangeForHubResponse::from_redis_value(&value)?;

                if res.status != 0 {
                    return Err(AppError::Internal { msg: res.msg });
                }
            }
            None => {
                for room in self.rooms(uid)? {
                    cmd.arg(lua_script::get_rooms_change_sha()?) //sha
                        .arg(0) //keys number
                        .arg(&ChangeRoomReq {
                            id: uid.to_string(),
                            name: None,
                            room: room.to_string(),
                            r#type: RoomChangeType::Join,
                        });

                    let value = redis_util::sync::conn()?.req_command(&cmd)?;

                    let res = RoomChangeForHubResponse::from_redis_value(&value)?;

                    if res.status != 0 {
                        return Err(AppError::Internal { msg: res.msg });
                    }
                }
            }
        }

        Ok(())
    }

    fn update_name(&self, uid: &str, uname: &str) -> AppResult<()> {
        let mut cmd = redis::cmd("evalsha");

        cmd.arg(lua_script::get_rooms_change_sha()?) //sha
            .arg(0) //keys number
            .arg(&ChangeRoomReq {
                id: uid.to_string(),
                name: Some(uname.to_string()),
                room: DEFAULT_ROOM.to_string(),
                r#type: RoomChangeType::UpdateName,
            });

        let value = redis_util::sync::conn()?.req_command(&cmd)?;

        let res = RoomChangeForHubResponse::from_redis_value(&value)?;

        if res.status != 0 {
            return Err(AppError::Internal { msg: res.msg });
        }

        Ok(())
    }
}
