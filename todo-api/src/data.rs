pub mod area;
pub mod project;
pub mod tag;
pub mod task;

use deadpool_postgres::{Manager, ManagerConfig, Pool};
use tokio_postgres::{Config, NoTls};

#[derive(Clone)]
pub struct AppState {
    db_pool: Pool,
}

impl AppState {
    pub fn with_pool(pool: Pool) -> Self {
        Self { db_pool: pool }
    }
}

pub async fn get_database_pool(
    host: String,
    port: u16,
    database: String,
    user: String,
    pass: String,
) -> Result<Pool, String> {
    let mut pg_config = Config::new();
    pg_config.host(host).port(port);
    pg_config.user(user).password(pass).dbname(database);

    let manager = Manager::from_config(pg_config, NoTls, ManagerConfig::default());

    let pool = Pool::builder(manager).max_size(16).build().unwrap();
    if pool.get().await.is_err() {
        return Err(String::from("Unable to access connection pool"));
    }

    Ok(pool)
}
