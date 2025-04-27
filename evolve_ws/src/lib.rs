pub mod chat_redis;
pub mod excute_lua_script;
pub mod store_redis;
use error::WSResult;
use evolve_redis::derive::{from_redis, to_redis};
use serde::{Deserialize, Serialize};
pub mod error;


pub const DEFAULT_ROOM: &str = "main";
pub const DEFAULT_ROOM_MANAGER_UID: &str = "system";
pub const DEFAULT_ROOM_MANAGER_UNAME: &str = "system";
pub const REDIS_WS_CHANNEL: &str = "evolve_axum_ws";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BroadCastType {
    Join,
    Quit,
    Message,
    ReName,
}

#[derive(Clone)]
#[from_redis]
#[to_redis]
pub struct BoradCastContent {
    pub ty: BroadCastType,
    #[allow(unused)]
    pub from_uid: String,
    pub from_uname: String,
    pub rooms: Vec<String>, // target room
    pub msg: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ReplyType {
    Notify,
    List,
    #[allow(unused)]
    Rooms,
}

#[derive(Clone, Debug)]
pub struct ReplyContent {
    pub ty: ReplyType,
    pub msg: Option<String>,
}

pub enum ContentType {
    BoradCast(BoradCastContent),
    Reply(ReplyContent),
}

pub trait Store {
    fn uname(&self, uid: &str) -> WSResult<String>;

    fn rooms(&self, uid: &str, page_index: usize, page_size: usize) -> WSResult<Vec<String>>;

    fn uids(&self, room: &str) -> WSResult<Vec<String>>;

    fn is_already_in_room(&self, uid: &str, room: &str) -> WSResult<bool>;

    fn join(&self, uid: &str, uname: &str, room: &str) -> WSResult<()>;

    fn quit(&self, uid: &str, room: Option<&str>) -> WSResult<()>;

    fn update_name(&self, uid: &str, uname: &str) -> WSResult<()>;

    fn update_room_order(&self, uid: &str, room: &str) -> WSResult<()>;
}
