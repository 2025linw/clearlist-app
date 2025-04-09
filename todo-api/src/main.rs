// New
mod error;
mod handler;
mod model;
mod route;
mod schema;
mod util;

use std::{env, sync::Arc};

use deadpool_postgres::{Object, Pool};
use dotenvy::dotenv;
use tokio::net::TcpListener;

use error::Error;
use route::create_api_router;
use util::get_database_pool;

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

// Server Main
#[tokio::main]
async fn main() {
    // Get .env environment variables
    dotenv().unwrap();

    let srv_port = env::var("SRV_PORT")
        .expect("SRV_PORT must be set")
        .parse::<u16>()
        .expect("SRV_PORT must be a valid port number");

    // Setup Database Connection Pool
    let (host, port, db_name, user, pass) = (
        env::var("DB_HOST").expect("DB_HOST must be set"),
        env::var("DB_PORT")
            .expect("DB_PORT must be set")
            .parse::<u16>()
            .expect("DB_PORT must be a valid port number"),
        env::var("DB_NAME").expect("DB_NAME must be set"),
        env::var("DB_USER").expect("DB_USER must be set"),
        env::var("DB_PASS").expect("DB_PASS must be set"),
    );
    let pool = match get_database_pool(host, port, db_name, user, pass).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);

            std::process::exit(1);
        }
    };

    let app_state = Arc::new(AppState::with_pool(pool));
    let router = create_api_router(app_state);

    let url = format!("localhost:{srv_port}");
    println!("Starting server at {}", url);
    let listener = TcpListener::bind(url).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
