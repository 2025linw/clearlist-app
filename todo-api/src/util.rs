pub mod sql_builder;

pub use sql_builder::*;

use axum_extra::extract::CookieJar;
use deadpool_postgres::{Manager, ManagerConfig, Pool, PoolError};
use tokio_postgres::{Config, NoTls};
use uuid::Uuid;

use crate::error::Error;

pub async fn get_database_pool(
    host: String,
    port: u16,
    database: String,
    user: String,
    pass: String,
) -> Result<Pool, PoolError> {
    let mut pg_config = Config::new();
    pg_config.host(host).port(port);
    pg_config.user(user).password(pass).dbname(database);

    let manager = Manager::from_config(pg_config, NoTls, ManagerConfig::default());

    let pool = Pool::builder(manager).max_size(16).build().unwrap();
    let _ = pool.get().await?;

    Ok(pool)
}

pub fn extract_user_id(cookies: &CookieJar) -> Result<Uuid, Error> {
    let cookie = match cookies.get("todo_app_user_id") {
        Some(c) => c,
        None => return Err(Error::InvalidRequest("User ID was not sent".to_string())),
    };

    match Uuid::try_parse(cookie.value()) {
        Ok(i) => Ok(i),
        Err(_) => Err(Error::InvalidRequest(
            "User ID was not a UUID format".to_string(),
        )),
    }
}
