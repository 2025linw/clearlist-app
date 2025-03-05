pub mod model;

mod handler;

use axum::{
    Router,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use handler::*;

use crate::error::Error;

pub fn api() -> Router {
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

    let router = Router::new()
        .nest("/tasks", task_api_router)
        .nest("/projects", project_api_router)
        .nest("/areas", area_api_router)
        .nest("/tags", tag_api_router);

    Router::new().nest("/api", router)
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
