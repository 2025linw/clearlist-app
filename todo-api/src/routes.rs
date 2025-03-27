mod handler;
mod utils;

use handler::*;

use axum::{
    Router,
    routing::{get, post},
};

pub fn user_api() -> Router {
    let user_api_router = Router::new();

    user_api_router
}

pub fn todo_api() -> Router {
    let task_api_router = Router::new()
        .route("/", post(create_task))
        .route(
            "/{id}",
            get(retrieve_task).put(update_task).delete(delete_task),
        )
        .route("/query", post(query_tasks));

    let project_api_router = Router::new()
        .route("/", post(create_project))
        .route(
            "/{id}",
            get(retrieve_project)
                .put(update_project)
                .delete(delete_project),
        )
        .route("/query", post(query_projects));

    let area_api_router = Router::new()
        .route("/", post(create_area))
        .route(
            "/{id}",
            get(retrieve_area).put(update_area).delete(delete_area),
        )
        .route("/query", post(query_areas));

    let tag_api_router = Router::new()
        .route("/", post(create_tag))
        .route(
            "/{id}",
            get(retrieve_tag).put(update_tag).delete(delete_tag),
        )
        .route("/query", post(query_tags));

    Router::new()
        .nest("/tasks", task_api_router)
        .nest("/projects", project_api_router)
        .nest("/areas", area_api_router)
        .nest("/tags", tag_api_router)
}
