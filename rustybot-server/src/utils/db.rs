use std::sync::{Arc};
use tokio::sync::{Mutex};

use sqlx::{MySql, Pool};

use super::config::Config;

lazy_static::lazy_static! {
  pub static ref DB_POOL: Arc<Mutex<Option<Pool<MySql>>>> = Arc::new(Mutex::new(None));
}

pub async fn create_pool() -> Result<sqlx::Pool<sqlx::mysql::MySql>, sqlx::Error> {
    let connection_string = Config::load().database.connection_string();
    sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&connection_string)
}

pub fn init_pool() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mut db = DB_POOL.lock().await;
            *db = Some(create_pool().await.unwrap());
        });
}