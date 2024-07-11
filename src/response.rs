use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct MsgResponse {
    pub msg: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct DataResponse<D>
where
    D: Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<usize>,
}

#[derive(Deserialize, IntoParams)]
pub struct Pagination {
    #[param(default=1)]
    pub index: i64,
    #[param(default=22)]
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

pub fn msg<T>(msg: T) -> MsgResponse
where
    T: AsRef<str>,
{
    MsgResponse {
        msg: msg.as_ref().to_string(),
    }
}

pub fn data<D>(data: D) -> DataResponse<D>
where
    D: Serialize,
{
    DataResponse {
        data: Some(data),
        total: None,
    }
}

pub fn data_with_total<D>(data: D, total: usize) -> DataResponse<D>
where
    D: Serialize,
{
    DataResponse {
        data: Some(data),
        total: Some(total),
    }
}
