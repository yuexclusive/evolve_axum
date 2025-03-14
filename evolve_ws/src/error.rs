use axum::{http::StatusCode, response::IntoResponse};
use redis::RedisError;
use thiserror::Error;

pub type WSResult<T> = std::result::Result<T, WSError>;

#[derive(Debug, Error)]
pub enum WSError {
    #[error(transparent)]
    Redis(#[from] RedisError),

    #[error("lua script excute error: {}",.msg)]
    LuaScriptExcuteError { msg: String },
}

impl IntoResponse for WSError {
    fn into_response(self) -> axum::response::Response {
        match self {
            WSError::Redis(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("redis error: {}", e),
            )
                .into_response(),
            WSError::LuaScriptExcuteError { msg } => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}
