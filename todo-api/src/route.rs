use std::sync::Arc;

use axum::{
    Extension, Router,
    http::Method,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_jwt::JwtLayer;

use crate::{
    AppState,
    handler::{area::*, auth::*, health_check_handler, project::*, tag::*, task::*},
};

pub fn create_api_router(app_state: Arc<AppState>) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user));

    let task_routes = Router::new()
        .route("/", post(query_task_handler))
        .route("/create", post(create_task_handler))
        .route(
            "/{id}",
            get(retrieve_task_handler)
                .patch(update_task_handler)
                .delete(delete_task_handler),
        );
    let project_routes = Router::new()
        .route("/", post(query_project_handler))
        .route("/create", post(create_project_handler))
        .route(
            "/{id}",
            get(retrieve_project_handler)
                .patch(update_project_handler)
                .delete(delete_project_handler),
        );
    let area_routes = Router::new()
        .route("/", post(query_area_handler))
        .route("/create", post(create_area_handler))
        .route(
            "/{id}",
            get(retrieve_area_handler)
                .patch(update_area_handler)
                .delete(delete_area_handler),
        );
    let tag_routes = Router::new()
        .route("/", post(query_tag_handler))
        .route("/create", post(create_tag_handler))
        .route(
            "/{id}",
            get(retrieve_tag_handler)
                .patch(update_tag_handler)
                .delete(delete_tag_handler),
        );

    let api_routes = Router::new()
        .route("/healthcheck", get(health_check_handler))
        .nest("/auth", auth_routes)
        .nest("/tasks", task_routes)
        .nest("/projects", project_routes)
        .nest("/areas", area_routes)
        .nest("/tags", tag_routes);

    Router::new().nest("/api", api_routes).layer(
        ServiceBuilder::new()
            .layer(CorsLayer::new().allow_methods([
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::PUT,
            ]))
            // .layer(JwtLayer::<Claim, _>::new())
            .layer(Extension(app_state)),
    )
}
