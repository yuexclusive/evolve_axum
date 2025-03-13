#![allow(static_mut_refs)]
use async_once::AsyncOnce;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::result::Result;

pub type SeaOrmResult<T, E = sea_orm::DbErr> = Result<T, E>;

static mut SEAORM_POOL: OnceCell<AsyncOnce<DatabaseConnection>> = OnceCell::new();

pub async fn conn() -> &'static DatabaseConnection {
    unsafe {
        SEAORM_POOL.get_or_init(|| -> AsyncOnce<DatabaseConnection> {
            AsyncOnce::new(async {
                let mut opt = ConnectOptions::new(&std::env::var("DATABASE_URL").unwrap());
                opt.max_connections(100)
                    .min_connections(5)
                    .acquire_timeout(std::time::Duration::from_secs(30))
                    .idle_timeout(std::time::Duration::from_secs(600));
                Database::connect(opt).await.unwrap()
            })
        })
    }
    .await
}

pub fn init() {
    dotenv::dotenv().unwrap();
    tracing::info!("seaorm init success");
}
