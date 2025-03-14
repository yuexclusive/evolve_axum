use crate::lua_script;
use crate::lua_script::RoomChangeForResponse;
use evolve_error::{AppError, AppResult};
use evolve_redis::redis::Commands;
use evolve_redis::redis::ConnectionLike;

use super::Store;
use super::DEFAULT_ROOM;
use serde::{Deserialize, Serialize};

use crate::lua_script::KEYS;
use redis::FromRedisValue;
use redis_encoding_derive::{from_redis, to_redis};

const USER_NAME_POSTFIX: &str = "_name";
const USER_ROOM_POSTFIX: &str = "_rooms";
const ROOM_USER_POSTFIX: &str = "_uids";

#[derive(Default)]
pub struct RedisStore;

#[from_redis]
// #[to_redis]
pub enum RoomChangeType {
    Join,
    Quit,
    UpdateName,
    UpdateRoomOrder,
}

#[to_redis]
#[from_redis]
pub struct ChangeRoomReq {
    pub id: String,
    pub name: Option<String>,
    pub room: String,
    pub r#type: RoomChangeType,
    pub service_id: Option<String>,
}

impl Store for RedisStore {
    fn uname(&self, uid: &str) -> AppResult<String> {
        let res = evolve_redis::sync::conn()?
            .get::<_, String>(format!("{}{}", uid, USER_NAME_POSTFIX))
            .unwrap_or_default();

        Ok(res)
    }

    fn rooms(&self, uid: &str, page_index: usize, page_size: usize) -> AppResult<Vec<String>> {
        let start = ((page_index - 1) * page_size) as isize;
        let stop = (page_index * page_size - 1) as isize;
        let res = evolve_redis::sync::conn()?.zrevrange(
            format!("{}{}", uid, USER_ROOM_POSTFIX),
            start,
            stop,
        )?;
        Ok(res)
    }

    fn uids(&self, room: &str) -> AppResult<Vec<String>> {
        let res = evolve_redis::sync::conn()?.smembers(format!("{}{}", room, ROOM_USER_POSTFIX))?;
        Ok(res)
    }

    fn is_already_in_room(&self, uid: &str, room: &str) -> AppResult<bool> {
        let res =
        evolve_redis::sync::conn()?.sismember(format!("{}{}", room, ROOM_USER_POSTFIX), uid)?;
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
                service_id: Some(KEYS.service_id.clone()),
            });

        let value = evolve_redis::sync::conn()?.req_command(&cmd)?;

        let res = RoomChangeForResponse::from_redis_value(&value)?;

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
                        service_id: Some(KEYS.service_id.clone()),
                    });

                let value = evolve_redis::sync::conn()?.req_command(&cmd)?;

                let res = RoomChangeForResponse::from_redis_value(&value)?;

                if res.status != 0 {
                    return Err(AppError::Internal { msg: res.msg });
                }
            }
            None => {
                for room in self.rooms(uid, 1, 1000)? {
                    cmd.arg(lua_script::get_rooms_change_sha()?) //sha
                        .arg(0) //keys number
                        .arg(&ChangeRoomReq {
                            id: uid.to_string(),
                            name: None,
                            room: room.to_string(),
                            r#type: RoomChangeType::Quit,
                            service_id: Some(KEYS.service_id.clone()),
                        });

                    let value = evolve_redis::sync::conn()?.req_command(&cmd)?;

                    let res = RoomChangeForResponse::from_redis_value(&value)?;

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
                service_id: None,
            });

        let value = evolve_redis::sync::conn()?.req_command(&cmd)?;

        let res = RoomChangeForResponse::from_redis_value(&value)?;

        if res.status != 0 {
            return Err(AppError::Internal { msg: res.msg });
        }

        Ok(())
    }

    fn update_room_order(&self, uid: &str, room: &str) -> AppResult<()> {
        let mut cmd = redis::cmd("evalsha");

        cmd.arg(lua_script::get_rooms_change_sha()?) //sha
            .arg(0) //keys number
            .arg(&ChangeRoomReq {
                id: uid.to_string(),
                name: None,
                room: room.to_string(),
                r#type: RoomChangeType::UpdateRoomOrder,
                service_id: None,
            });

        let value = evolve_redis::sync::conn()?.req_command(&cmd)?;

        let res = RoomChangeForResponse::from_redis_value(&value)?;

        if res.status != 0 {
            return Err(AppError::Internal { msg: res.msg });
        }

        Ok(())
    }
}
