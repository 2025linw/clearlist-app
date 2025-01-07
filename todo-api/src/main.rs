use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::{Manager, ManagerConfig, Pool};
use std::env;
use tokio_postgres::NoTls;

pub mod data;
pub mod models;

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
    let pool = data::get_database_pool(
        env::var("DB_HOST").unwrap(),
        env::var("DB_PORT").unwrap().parse::<u16>().unwrap(),
        env::var("DB_NAME").unwrap(),
        env::var("DB_USER").unwrap(),
        env::var("DB_PASS").unwrap(),
    )
    .await
    .unwrap();

    let state = data::AppState::with_pool(pool);

    // Setup Web Server Routing
    let api_router = Router::new()
        .route("/tasks", post(data::task::create_task))
        .route(
            "/tasks/:id",
            get(data::task::retrieve_task)
                .put(data::task::update_task)
                .delete(data::task::delete_task),
        )
        .route("/tasks/query", post(data::task::query_tasks))
        .route("/projects", post(data::project::create_project))
        .route(
            "/projects/:id",
            get(data::project::retrieve_project)
                .put(data::project::update_project)
                .delete(data::project::delete_project),
        )
        .route("/projects/query", post(data::project::query_projects))
        .route("/areas", post(data::area::create_area))
        .route(
            "/areas/:id",
            get(data::area::retrieve_area)
                .put(data::area::update_area)
                .delete(data::area::delete_area),
        )
        .route("/tags", post(data::tag::create_tag))
        .route(
            "/tags/:id",
            get(data::tag::retrieve_tag)
                .put(data::tag::update_tag)
                .delete(data::tag::delete_tag),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("localhost:{PORT}"))
        .await
        .unwrap();

    // Setup Server
    axum::serve(listener, api_router).await.unwrap();
}
