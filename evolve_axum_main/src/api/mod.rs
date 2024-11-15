pub mod auth;
pub mod user;
pub mod version;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

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

#[derive(Deserialize, IntoParams)]
pub struct Pagination {
    #[param(example = 1)]
    pub index: i64,
    #[param(example = 20)]
    pub size: i64,
}