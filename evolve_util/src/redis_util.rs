#![allow(unused)]
use std::future::Future;

use once_cell::sync::OnceCell;
pub use redis;
use redis::{
    aio::{ConnectionLike, MultiplexedConnection, PubSub},
    AsyncCommands, Client, ConnectionAddr, ConnectionInfo, IntoConnectionInfo, RedisConnectionInfo,
    RedisResult, ToRedisArgs,
};
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::oneshot::{self, Receiver};
use tokio_stream::StreamExt;

use evolve_error::AppError;
pub mod derive {
    pub use redis_encoding_derive::{from_redis, to_redis};
}

static CLIENT: OnceCell<Client> = OnceCell::new();

static mut CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Clone, Default, Debug)]
struct Config {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl IntoConnectionInfo for Config {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        Ok(ConnectionInfo {
            addr: ConnectionAddr::Tcp(self.host, self.port),
            redis: RedisConnectionInfo {
                username: self.username,
                password: self.password,
                ..Default::default()
            },
        })
    }
}

pub async fn init(
    host: impl AsRef<str>,
    port: u16,
    username: Option<String>,
    password: Option<String>,
) {
    unsafe {
        CONFIG.get_or_init(|| Config {
            host: host.as_ref().to_owned(),
            port,
            username,
            password,
        });
    }

    match ping().await {
        Ok(redis::Value::Status(ref v)) => {
            if v == "PONG" {
                tracing::info!("redis init success");
            } else {
                panic!("redis init failed, status: {}", v);
            }
        }
        Err(e) => {
            panic!("redis init failed, error: {}", e)
        }
        other => {
            panic!("redis init failed, other: {:?}", other)
        }
    }

    match sync::ping() {
        Ok(redis::Value::Status(ref v)) => {
            if v == "PONG" {
                tracing::info!("redis init success");
            } else {
                panic!("redis init failed, status: {}", v);
            }
        }
        Err(e) => {
            panic!("redis init failed, error: {}", e)
        }
        other => {
            panic!("redis init failed, other: {:?}", other)
        }
    }
}

pub async fn conn() -> RedisResult<MultiplexedConnection> {
    let cfg = unsafe { CONFIG.get_unchecked() };
    let client = CLIENT.get_or_init(|| redis::Client::open(cfg.clone()).unwrap());
    client.get_multiplexed_async_connection().await
}

pub async fn pubsub() -> RedisResult<PubSub> {
    let cfg = unsafe { CONFIG.get_unchecked() };
    let client = CLIENT.get_or_init(|| redis::Client::open(cfg.clone()).unwrap());
    let res = client.get_async_pubsub().await?;
    Ok(res)
}

pub async fn publish<K, V>(channel: K, v: V) -> RedisResult<()>
where
    K: ToRedisArgs + Send + Sync,
    V: ToRedisArgs + Send + Sync,
{
    conn().await?.publish(channel, v).await
}

pub async fn ping() -> RedisResult<redis::Value> {
    conn().await?.req_packed_command(&redis::cmd("PING")).await
}

pub mod sync {
    use redis::{Connection, RedisResult};
    use redis::{ConnectionLike, PubSubCommands};

    use super::{CLIENT, CONFIG};

    pub fn conn() -> RedisResult<Connection> {
        let cfg = unsafe { CONFIG.get_unchecked() };
        let client = CLIENT.get_or_init(|| redis::Client::open(cfg.clone()).unwrap());
        let conn = client.get_connection()?;
        Ok(conn)
    }

    pub fn ping() -> RedisResult<redis::Value> {
        conn()?.req_command(&redis::cmd("PING"))
    }
}
