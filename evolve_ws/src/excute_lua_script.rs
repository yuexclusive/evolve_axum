#![allow(unused)]
use crate::error::{WSError, WSResult};
use dotenv::dotenv;
use evolve_redis::derive::from_redis;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use redis::FromRedisValue;
use redis::{Commands, ConnectionLike};
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
        Err(_) => true,
    },
    service_id: "MY_SERVICE_ID".to_string(),
});

pub fn get_rooms_change_sha<'a>() -> WSResult<&'a str> {
    Ok(ROOMS_CHANGE_SHA.get_or_try_init(|| load_rooms_change())?)
}

pub fn get_clear_sessions_sha<'a>() -> WSResult<&'a str> {
    Ok(CLEAR_SESSIONS_SHA.get_or_try_init(|| load_clear_sessions())?)
}

#[allow(dependency_on_unit_never_type_fallback)]
fn load_rooms_change() -> WSResult<String> {
    if !KEYS.redis_force_refresh_script_sha {
        let sha: String = evolve_redis::sync::conn()?
            .get(&KEYS.rooms_change_sha_key)
            .unwrap_or_default();
        if !sha.is_empty() {
            return Ok(sha);
        }
    }
    let mut conn = evolve_redis::sync::conn()?;

    let mut cmd = redis::cmd("script");

    let cmd_str = format!(
        "{}{}",
        std::include_str!("../static/lua_scripts/json.lua"),
        std::include_str!("../static/lua_scripts/ws_change.lua")
    );

    cmd.arg("load").arg(cmd_str);

    let value = conn.req_command(&cmd)?;

    let res = String::from_redis_value(&value)?;
    evolve_redis::sync::conn()?.set(&KEYS.rooms_change_sha_key, &res)?;
    Ok(res)
}

#[allow(dependency_on_unit_never_type_fallback)]
fn load_clear_sessions() -> WSResult<String> {
    if !KEYS.redis_force_refresh_script_sha {
        let sha: String = evolve_redis::sync::conn()?
            .get(&KEYS.clear_sessions_sha_key)
            .unwrap_or_default();
        if !sha.is_empty() {
            return Ok(sha);
        }
    }
    let mut conn = evolve_redis::sync::conn()?;
    let mut cmd = redis::cmd("script");

    let cmd_str = format!(
        "{}{}",
        std::include_str!("../static/lua_scripts/json.lua"),
        std::include_str!("../static/lua_scripts/clear_sessions.lua")
    );

    cmd.arg("load").arg(cmd_str);

    let value = conn.req_command(&cmd)?;
    let res = String::from_redis_value(&value)?;
    evolve_redis::sync::conn()?.set(&KEYS.clear_sessions_sha_key, &res)?;
    Ok(res)
}

pub fn clear_sessions() -> WSResult<()> {
    let mut cmd = redis::cmd("evalsha");

    let service_id = &KEYS.service_id;

    let sha = get_clear_sessions_sha()?;

    tracing::info!("clear_sessions lua sha: {}", sha);

    cmd.arg(sha) //sha
        .arg(0) //keys number
        .arg(service_id);

    let value = evolve_redis::sync::conn()?.req_command(&cmd)?;

    let res = RoomChangeForResponse::from_redis_value(&value)?;

    if res.status != 0 {
        return Err(WSError::LuaScriptExcuteError { msg: res.msg }.into());
    }
    Ok(())
}
