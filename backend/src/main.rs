mod error;
mod handler;
mod model;
mod route;
mod schema;
mod util;

use std::{env, net::SocketAddr};

use axum::{Router, extract::FromRef, http::Method};
use axum_jwt_auth::JwtDecoderState;
use deadpool_postgres::{Object, Pool};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
// use tracing::{error, info, warn};
use tracing::info;
use tracing_subscriber::EnvFilter;

use error::Error;
use route::create_api_router;
use schema::auth::Claim;
use util::{auth::create_decoder, get_database_pool};

#[derive(Clone, FromRef)]
pub struct AppState {
    decoder: JwtDecoderState<Claim>,
    db_pool: Pool,
}

impl AppState {
    #[inline]
    pub async fn get_conn(&self) -> Result<Object, Error> {
        return Ok(self.db_pool.get().await?);
    }
}

// Server Main
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    // Get .env environment variables
    info!("Getting environment variables");
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

    // Get decoder
    let decoder = match create_decoder() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("unable to create decoder: {:?}", e);

            std::process::exit(1);
        }
    };

    // Setup app state
    let app_state = AppState {
        decoder,
        db_pool: pool,
    };

    let router = Router::new().nest(
        "/api",
        create_api_router().layer(ServiceBuilder::new().layer(CorsLayer::new().allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
        ]))),
    );

    let url = format!("localhost:{srv_port}");
    let listener = TcpListener::bind(&url).await.unwrap();

    info!("Starting server at {}", url);
    axum::serve(
        listener,
        router
            .with_state(app_state)
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("failed to start server");
}
