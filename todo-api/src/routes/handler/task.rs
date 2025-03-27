use axum::{
    Json,
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use deadpool_postgres::Object;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        db::TaskModel,
        request::{
            TaskCreateRequest, TaskDeleteRequest, TaskQueryRequest, TaskRetrieveRequest,
            TaskUpdateRequest,
        },
    },
    routes::utils::extract_user_id,
    storage::db,
};

// POST /api/tasks
pub async fn create_task(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Json(details): Json<TaskCreateRequest>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TaskCreateRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };

    let task_id: Uuid = match db::insert(&mut conn, &details).await {
        Ok(i) => i,
        Err(e) => return e.into_response(),
    };

    (StatusCode::CREATED, task_id.to_string()).into_response()
}

// GET /api/tasks/:id
pub async fn retrieve_task(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(task_id): Path<uuid::Uuid>,
) -> Response {
    let conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TaskRetrieveRequest = TaskRetrieveRequest::default();
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_task_id(task_id);

    let row_opt: Option<TaskModel> = match db::select_one(&conn, &details).await {
        Ok(o) => o,
        Err(e) => return e.into_response(),
    };

    match row_opt {
        Some(r) => Json(r).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// PUT /api/tasks/:id
pub async fn update_task(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(task_id): Path<uuid::Uuid>,
    Json(details): Json<TaskUpdateRequest>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TaskUpdateRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_task_id(task_id);

    let row_opt: Option<TaskModel> = match db::update(&mut conn, &details).await {
        Ok(r) => r,
        Err(e) => return e.into_response(),
    };

    match row_opt {
        Some(r) => Json(r).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// DELETE /api/tasks/:id
pub async fn delete_task(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(task_id): Path<uuid::Uuid>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TaskDeleteRequest = TaskDeleteRequest::default();
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_task_id(task_id);

    let success: bool = match db::delete(&mut conn, &details).await {
        Ok(b) => b,
        Err(e) => return e.into_response(),
    };

    if success {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

// POST /api/tasks/query
pub async fn query_tasks(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Query(params): Query<HashMap<String, String>>,
    Json(details): Json<TaskQueryRequest>,
) -> Response {
    let conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TaskQueryRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    if let Some(query) = params.get("query") {
        details.set_search_query(query.to_owned());
    }

    let task_models: Vec<TaskModel> = match db::select_all(&conn, &details).await {
        Ok(v) => v,
        Err(e) => return e.into_response(),
    };

    Json(task_models).into_response()
}
