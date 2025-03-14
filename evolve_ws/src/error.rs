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
