use axum::{
    extract::{Path, Query as URLQuery, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    database::TaskModel,
    database::AppState,
    request::{
        api::{Create, Delete, InfoBuilder, Query, Retrieve, Update},
        extract_user_id,
        task::*,
    },
    response::{COOKIE_GET_ERROR, SERVER_POOL_ERROR},
};

// POST /api/tasks
pub async fn create(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(details): Json<TaskPostRequest>,
) -> Response {
    let state = state.clone();
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    let mut details = details;
    match extract_user_id(&jar) {
        Some(i) => details.user_id(i),
        None => {
            eprintln!("{}", COOKIE_GET_ERROR);

            return (StatusCode::INTERNAL_SERVER_ERROR, COOKIE_GET_ERROR).into_response();
        }
    };

    let task_id: Uuid = match details.insert_query(&mut conn, None).await {
        Ok(r) => r.get(TaskModel::ID),
        Err(e) => {
            eprintln!("{e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to insert task into database",
            )
                .into_response();
        }
    };

    (StatusCode::CREATED, task_id.to_string()).into_response()
}

// GET /api/tasks/:id
pub async fn retrieve(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(task_id): Path<uuid::Uuid>,
) -> Response {
    let state = state.clone();
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    let details = TaskGetRequest {};

    let mut info_builder = InfoBuilder::new();
    match extract_user_id(&jar) {
        Some(i) => info_builder.user_id(i),
        None => {
            eprintln!("{}", COOKIE_GET_ERROR);

            return (StatusCode::INTERNAL_SERVER_ERROR, COOKIE_GET_ERROR).into_response();
        }
    };
    info_builder.obj_id(task_id);

    let info = info_builder.build();
    let row_opt = match details.select_query(&mut conn, Some(info)).await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to retrieve task from database",
            )
                .into_response();
        }
    };

    let task_model: TaskModel = match row_opt {
        Some(r) => TaskModel::from(r),
        None => {
            eprintln!("Task not found in database");

            return (StatusCode::NOT_FOUND).into_response();
        }
    };

    Json(task_model).into_response()
}

// PUT /api/tasks/:id
pub async fn update(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(task_id): Path<uuid::Uuid>,
    Json(details): Json<TaskPutRequest>,
) -> Response {
    let state = state.clone();
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    let mut info_builder = InfoBuilder::new();
    match extract_user_id(&jar) {
        Some(i) => info_builder.user_id(i),
        None => {
            eprintln!("{}", COOKIE_GET_ERROR);

            return (StatusCode::INTERNAL_SERVER_ERROR, COOKIE_GET_ERROR).into_response();
        }
    };
    info_builder.obj_id(task_id);

    let info = info_builder.build();
    let row_opt = match details.update_query(&mut conn, Some(info)).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to update task in database",
            )
                .into_response();
        }
    };

    let task_model: TaskModel = match row_opt {
        Some(r) => TaskModel::from(r),
        None => {
            eprintln!("Unable to find task with id");

            return (StatusCode::NOT_FOUND).into_response();
        }
    };

    Json(task_model).into_response()
}

// DELETE /api/tasks/:id
pub async fn delete(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(task_id): Path<uuid::Uuid>,
) -> Response {
    let state = state.clone();
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    let details = TaskDeleteRequest {};

    let mut info_builder = InfoBuilder::new();
    match extract_user_id(&jar) {
        Some(i) => info_builder.user_id(i),
        None => {
            eprintln!("{}", COOKIE_GET_ERROR);

            return (StatusCode::INTERNAL_SERVER_ERROR, COOKIE_GET_ERROR).into_response();
        }
    };
    info_builder.obj_id(task_id);

    let info = info_builder.build();
    let success = match details.delete_query(&mut conn, Some(info)).await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to delete task from database",
            )
                .into_response();
        }
    };

    if !success {
        return (StatusCode::NOT_FOUND).into_response();
    }

    return (StatusCode::NO_CONTENT).into_response();
}

// POST /api/tasks/query
pub async fn query(
    State(state): State<AppState>,
    jar: CookieJar,
    URLQuery(params): URLQuery<HashMap<String, String>>,
    Json(details): Json<TaskQueryRequest>,
) -> Response {
    let state = state.clone();
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    let mut info_builder = InfoBuilder::new();
    match extract_user_id(&jar) {
        Some(i) => info_builder.user_id(i),
        None => {
            eprintln!("{}", COOKIE_GET_ERROR);

            return (StatusCode::INTERNAL_SERVER_ERROR, COOKIE_GET_ERROR).into_response();
        }
    };
    if let Some(query) = params.get("query") {
        info_builder.query(query.clone());
    }

    let info = info_builder.build();
    let rows = match details.query(&mut conn, Some(info)).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to query tasks from database",
            )
                .into_response();
        }
    };

    let task_models: Vec<TaskModel> = rows.iter().map(|r| TaskModel::from(r.clone())).collect();

    Json(task_models).into_response()
}
