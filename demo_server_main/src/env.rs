#![allow(unused)]
use chrono::format::parse;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Env {
    pub port: u16,
    pub rust_log: String,
    pub jwt_secret: String,
    pub token_valid_duraiton: usize,
    pub meilisearch_address: String,
    pub meilisearch_api_key: String,
    pub email_username: String,
    pub email_password: String,
    pub email_relay: String,
    pub email_port: u16,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_username: Option<String>,
    pub redis_password: Option<String>,
    pub redis_force_refresh_script_sha: bool,
}

pub static ENV: Lazy<Env> = Lazy::new(|| {
    let port = dotenv::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let rust_log = dotenv::var("RUST_LOG")
        .ok()
        .unwrap_or_else(|| "demo_server_main=debug,tower_http=debug,axum::rejection=trace".into());

    let jwt_secret = dotenv::var("JWT_SECRET").ok().unwrap_or_default();

    let token_valid_duraiton = dotenv::var("TOKEN_VALID_DURATION")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(3600 * 24);

    let meilisearch_address = dotenv::var("MEILISEARCH_ADDRESS").ok().unwrap_or_default();

    let meilisearch_api_key = dotenv::var("MEILISEARCH_API_KEY").ok().unwrap_or_default();

    let email_username = dotenv::var("EMAIL_USERNAME").ok().unwrap_or_default();

    let email_password = dotenv::var("EMAIL_PASSWORD").ok().unwrap_or_default();

    let email_relay = dotenv::var("EMAIL_RELAY").ok().unwrap_or_default();

    let email_port = dotenv::var("EMAIL_PORT")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(465);

    let redis_host = dotenv::var("REDIS_HOST")
        .ok()
        .unwrap_or("127.0.0.1".to_string());

    let redis_port = dotenv::var("REDIS_PORT")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(6379);

    let redis_username = dotenv::var("REDIS_USERNAME").ok();

    let redis_password = dotenv::var("REDIS_PASSWORD").ok();

    let redis_force_refresh_script_sha = dotenv::var("REDIS_FORCE_REFRESH_SCRIPT_SHA")
        .unwrap_or("false".to_string())
        .parse::<bool>()
        .unwrap();

    let env = Env {
        port,
        rust_log,
        jwt_secret,
        token_valid_duraiton,
        meilisearch_address,
        meilisearch_api_key,
        email_username,
        email_password,
        email_relay,
        email_port,
        redis_host,
        redis_port,
        redis_username,
        redis_password,
        redis_force_refresh_script_sha,
    };

    println!("Server start with env:\n{:#?}", env);
    env
});
