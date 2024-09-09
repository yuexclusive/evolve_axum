use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct MsgResp {
    pub msg: String,
}

#[derive(Deserialize, IntoParams)]
pub struct Pagination {
    #[param(default = 1)]
    pub index: i64,
    #[param(default = 22)]
    pub size: i64,
}

impl Pagination {
    pub fn skip(&self) -> i64 {
        self.index.checked_sub(1).unwrap_or(0) * self.size
    }

    pub fn take(&self) -> i64 {
        self.size.max(0)
    }
}

pub fn msg<T>(msg: T) -> MsgResp
where
    T: AsRef<str>,
{
    MsgResp {
        msg: msg.as_ref().to_string(),
    }
}
