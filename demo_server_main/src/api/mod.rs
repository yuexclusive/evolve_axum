pub mod auth;
pub mod user;
pub mod version;
use serde::Serialize;
use utoipa::ToSchema;
pub mod paging;
pub mod file;
pub mod stream;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct MsgResp {
    pub msg: String,
}

pub fn msg<T>(msg: T) -> MsgResp
where
    T: AsRef<str>,
{
    MsgResp {
        msg: msg.as_ref().to_string(),
    }
}