#![allow(unused)]
use crate::error::AppResult;
use once_cell::sync::OnceCell;
use redis::{ConnectionLike, FromRedisValue};

static ROOMS_CHANGE_SHA: OnceCell<String> = OnceCell::new();

pub fn get_rooms_change_sha<'a>() -> AppResult<&'a str> {
    Ok(ROOMS_CHANGE_SHA.get_or_try_init(|| load_rooms_change())?)
}

fn load_rooms_change() -> AppResult<String> {
    let mut conn = crate::redis_util::sync::conn()?;
    let files = ["json.lua", "ws_change.lua"];
    let cmd_str: String = files
        .iter()
        .map(|&name| std::fs::read_to_string(format!("static/lua_scripts/{}", name)).unwrap())
        .collect();

    let mut cmd = redis::cmd("script");

    cmd.arg("load").arg(cmd_str);

    let value = conn.req_command(&cmd)?;

    let res = String::from_redis_value(&value)?;
    Ok(res)
}