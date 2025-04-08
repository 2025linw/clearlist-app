use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handler::{area::*, health_check_handler, project::*, tag::*, task::*, user::*},
};

pub fn create_api_router(app_state: Arc<AppState>) -> Router {
    let user_routes = Router::new();

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
        .nest("/users", user_routes)
        .nest("/tasks", task_routes)
        .nest("/projects", project_routes)
        .nest("/areas", area_routes)
        .nest("/tags", tag_routes)
        .with_state(app_state);

    Router::new().nest("/api", api_routes)
}
