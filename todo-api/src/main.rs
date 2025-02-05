pub mod database;
pub mod handler;
pub mod request;
pub mod response;

use axum::{
    routing::{get, post},
    Router,
};
use std::env;

use handler::{area, project, tag, task};

// Server Constants
const PORT: u16 = 8082;

/*
 * Server Startup
 */

#[tokio::main]
async fn main() {
    // Setup Environment Variables
    dotenvy::dotenv().unwrap();

    // Setup Database Connection Pool
    let pool = match database::get_database_pool(
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

    let state = database::AppState::with_pool(pool);

    // Setup Web Server Routing
    let task_api_router = Router::new()
        .route("/", post(task::create))
        .route(
            "/{id}",
            get(task::retrieve).put(task::update).delete(task::delete),
        )
        .route("/query", post(task::query));
    let project_api_router = Router::new()
        .route("/", post(project::create))
        .route(
            "/{id}",
            get(project::retrieve)
                .put(project::update)
                .delete(project::delete),
        )
        .route("/query", post(project::query));
    let area_api_router = Router::new()
        .route("/", post(area::create))
        .route(
            "/{id}",
            get(area::create).put(area::update).delete(area::delete),
        )
        .route("/query", post(area::query));
    let tag_api_router = Router::new()
        .route("/", post(tag::create))
        .route(
            "/{id}",
            get(tag::retrieve).put(tag::update).delete(tag::delete),
        )
        .route("/query", post(tag::query));

    let api_router = Router::new()
        .nest("/tasks", task_api_router)
        .nest("/projects", project_api_router)
        .nest("/areas", area_api_router)
        .nest("/tags", tag_api_router)
        .with_state(state);

    let router = Router::new().nest("/api", api_router);

    let listener = tokio::net::TcpListener::bind(format!("localhost:{PORT}"))
        .await
        .unwrap();

    // Setup Server
    axum::serve(listener, router).await.unwrap();
}
