use redis_encoding_derive::{from_redis, to_redis};
use serde::{Deserialize, Serialize};

use crate::error::AppResult;

pub mod chat_redis;
pub mod store_redis;

pub const DEFAULT_ROOM: &str = "main";
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
    fn uname(&self, uid: &str) -> AppResult<String>;

    fn rooms(&self, uid: &str, page_index: usize, page_size: usize) -> AppResult<Vec<String>>;

    fn uids(&self, room: &str) -> AppResult<Vec<String>>;

    fn is_already_in_room(&self, uid: &str, room: &str) -> AppResult<bool>;

    fn join(&self, uid: &str, uname: &str, room: &str) -> AppResult<()>;

    fn quit(&self, uid: &str, room: Option<&str>) -> AppResult<()>;

    fn update_name(&self, uid: &str, uname: &str) -> AppResult<()>;

    fn update_room_order(&self, uid: &str, room: &str) -> AppResult<()>;
}
