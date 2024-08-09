use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum BroadCastType {
    Join,
    Quit,
    Message,
    ReName,
}

#[derive(Clone, Debug)]
pub struct BoradCastContent {
    pub ty: BroadCastType,
    #[allow(unused)]
    pub from_uid: String,
    pub from_uname: String,
    pub rooms: HashSet<String>, // target room
    pub msg: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ReplyType {
    Notify,
    List,
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
