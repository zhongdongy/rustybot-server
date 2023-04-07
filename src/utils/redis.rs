use redis::{Client, Connection, RedisError};

use super::config::Config;

pub fn get_connection() -> Result<Connection, RedisError> {
    let config = Config::load().unwrap();

    let client = Client::open(format!(
        "redis://{}:{}@{}:{}/{}",
        config.redis.username.unwrap_or("default".to_string()),
        config.redis.password,
        config.redis.host,
        config.redis.port,
        config.redis.select.unwrap_or(0)
    ))
    .unwrap();
    client.get_connection()
}
