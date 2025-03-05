mod routes;
mod storage;

mod error;
mod util;

use axum::Extension;
use deadpool_postgres::{Manager, ManagerConfig, Object, Pool, PoolError};
use dotenvy::dotenv;
use error::Error;
use std::{env, sync::Arc};
use tokio_postgres::{Config, NoTls};

use routes::api;

// TODO: Get logging library/middleware

// Server Main
#[tokio::main]
async fn main() {
    // Setup Environment Variables
    dotenv().unwrap();

    let port = env::var("SRV_PORT").unwrap().parse::<u16>().unwrap();

    // Setup Database Connection Pool
    let pool = match get_database_pool(
        env::var("DB_HOST").unwrap(),
        env::var("DB_PORT").unwrap().parse::<u16>().unwrap(),
        env::var("DB_NAME").unwrap(),
        env::var("DB_USER").unwrap(),
        env::var("DB_PASS").unwrap(),
    )
    .await
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error starting API server: {}", e);

            panic!()
        }
    };

    let shared_state = Arc::new(AppState::with_pool(pool));

    let listener = tokio::net::TcpListener::bind(format!("localhost:{port}"))
        .await
        .unwrap();
    let router = api().layer(Extension(shared_state));
    axum::serve(listener, router).await.unwrap();
}

pub struct AppState {
    db_pool: Pool,
}

impl AppState {
    pub fn with_pool(pool: Pool) -> Self {
        Self { db_pool: pool }
    }

    #[inline]
    pub async fn get_conn(&self) -> Result<Object, Error> {
        return Ok(self.db_pool.get().await?);
    }
}

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
