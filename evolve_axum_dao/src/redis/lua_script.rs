#![allow(unused)]
use dotenv::dotenv;
use evolve_error::{AppError, AppResult};
use evolve_util::redis_util;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use redis::FromRedisValue;
use redis::{Commands, ConnectionLike};
use redis_encoding_derive::from_redis;
use serde::{Deserialize, Serialize};

#[from_redis]
pub struct RoomChangeForResponse {
    pub status: i8,
    pub msg: String,
}

static ROOMS_CHANGE_SHA: OnceCell<String> = OnceCell::new();

static CLEAR_SESSIONS_SHA: OnceCell<String> = OnceCell::new();

// const ROOMS_CHANGE_SHA_KEY: &str = "rooms_change_sha";
// const CLEAR_SESSIONS_SHA_KEY: &str = "clear_sessions_sha";

pub struct Keys {
    rooms_change_sha_key: &'static str,
    clear_sessions_sha_key: &'static str,
    redis_force_refresh_script_sha: bool,
    pub service_id: String,
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| Keys {
    rooms_change_sha_key: "ROOMS_CHANGE_SHA_KEY",
    clear_sessions_sha_key: "CLEAR_SESSIONS_SHA_KEY",
    redis_force_refresh_script_sha: match dotenv::var("REDIS_FORCE_REFRESH_SCRIPT_SHA") {
        Ok(v) => v == "true",
        Err(_) => false,
    },
    service_id: uuid::Uuid::new_v4().to_string(),
});

pub fn get_rooms_change_sha<'a>() -> AppResult<&'a str> {
    Ok(ROOMS_CHANGE_SHA.get_or_try_init(|| load_rooms_change())?)
}

pub fn get_clear_sessions_sha<'a>() -> AppResult<&'a str> {
    Ok(CLEAR_SESSIONS_SHA.get_or_try_init(|| load_clear_sessions())?)
}

#[allow(dependency_on_unit_never_type_fallback)]
fn load_rooms_change() -> AppResult<String> {
    if !KEYS.redis_force_refresh_script_sha {
        let sha: String = redis_util::sync::conn()?
            .get(&KEYS.rooms_change_sha_key)
            .unwrap_or_default();
        if !sha.is_empty() {
            return Ok(sha);
        }
    }
    let mut conn = redis_util::sync::conn()?;
    let files = ["json.lua", "ws_change.lua"];
    let cmd_str: String = files
        .iter()
        .map(|&name| std::fs::read_to_string(format!("static/lua_scripts/{}", name)).unwrap())
        .collect();

    let mut cmd = redis::cmd("script");

    cmd.arg("load").arg(cmd_str);

    let value = conn.req_command(&cmd)?;

    let res = String::from_redis_value(&value)?;
    redis_util::sync::conn()?.set(&KEYS.rooms_change_sha_key, &res)?;
    Ok(res)
}

#[allow(dependency_on_unit_never_type_fallback)]
fn load_clear_sessions() -> AppResult<String> {
    if !KEYS.redis_force_refresh_script_sha {
        let sha: String = redis_util::sync::conn()?
            .get(&KEYS.clear_sessions_sha_key)
            .unwrap_or_default();
        if !sha.is_empty() {
            return Ok(sha);
        }
    }
    let mut conn = redis_util::sync::conn()?;
    let files = ["json.lua", "clear_sessions.lua"];
    let cmd_str: String = files
        .iter()
        .map(|&name| std::fs::read_to_string(format!("static/lua_scripts/{}", name)).unwrap())
        .collect();

    let mut cmd = redis::cmd("script");

    cmd.arg("load").arg(cmd_str);

    let value = conn.req_command(&cmd)?;
    let res = String::from_redis_value(&value)?;
    redis_util::sync::conn()?.set(&KEYS.clear_sessions_sha_key, &res)?;
    Ok(res)
}

pub fn clear_sessions() -> AppResult<()> {
    let mut cmd = redis::cmd("evalsha");

    let service_id = &KEYS.service_id;

    let sha = get_clear_sessions_sha()?;

    tracing::debug!("clear_sessions lua sha: {}", sha);

    cmd.arg(sha) //sha
        .arg(0) //keys number
        .arg(service_id);

    let value = redis_util::sync::conn()?.req_command(&cmd)?;

    let res = RoomChangeForResponse::from_redis_value(&value)?;

    if res.status != 0 {
        return Err(AppError::Internal { msg: res.msg });
    }
    Ok(())
}
