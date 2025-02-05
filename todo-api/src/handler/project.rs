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
    database::ProjectModel,
    database::AppState,
    request::{
        api::{Create, Delete, InfoBuilder, Query, Retrieve, Update},
        extract_user_id,
        project::*,
    },
    response::{COOKIE_GET_ERROR, SERVER_POOL_ERROR},
};

// POST /api/projects
pub async fn create(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(details): Json<ProjectPostRequest>,
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

    let info = info_builder.build();
    let project_id: Uuid = match details.insert_query(&mut conn, Some(info)).await {
        Ok(r) => r.get(ProjectModel::ID),
        Err(e) => {
            eprintln!("{e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to insert project to database",
            )
                .into_response();
        }
    };

    (StatusCode::CREATED, project_id.to_string()).into_response()
}

// GET /api/projects/:id
pub async fn retrieve(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(project_id): Path<uuid::Uuid>,
) -> Response {
    let state = state.clone();
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    let details = ProjectGetRequest {};

    let mut info_builder = InfoBuilder::new();
    match extract_user_id(&jar) {
        Some(i) => info_builder.user_id(i),
        None => {
            eprintln!("{}", COOKIE_GET_ERROR);

            return (StatusCode::INTERNAL_SERVER_ERROR, COOKIE_GET_ERROR).into_response();
        }
    };
    info_builder.obj_id(project_id);

    let info = info_builder.build();
    let row_opt = match details.select_query(&mut conn, Some(info)).await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to retrieve project from database",
            )
                .into_response();
        }
    };

    let project_model: ProjectModel = match row_opt {
        Some(r) => ProjectModel::from(r),
        None => {
            eprintln!("Task not found in database");

            return (StatusCode::NOT_FOUND).into_response();
        }
    };

    Json(project_model).into_response()
}

// PUT /api/projects/:id
pub async fn update(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(project_id): Path<uuid::Uuid>,
    Json(details): Json<ProjectPutRequest>,
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
    info_builder.obj_id(project_id);

    let info = info_builder.build();
    let row_opt = match details.update_query(&mut conn, Some(info)).await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to update project in database",
            )
                .into_response();
        }
    };

    let project_model = match row_opt {
        Some(r) => ProjectModel::from(r),
        None => {
            eprintln!("Unable to find project with id");

            return (StatusCode::NOT_FOUND).into_response();
        }
    };

    Json(project_model).into_response()
}

// DELETE /api/projects/:id
pub async fn delete(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(project_id): Path<uuid::Uuid>,
) -> Response {
    let state = state.clone();
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    let details = ProjectDeleteRequest {};

    let mut info_builder = InfoBuilder::new();
    match extract_user_id(&jar) {
        Some(i) => info_builder.user_id(i),
        None => {
            eprintln!("{}", COOKIE_GET_ERROR);

            return (StatusCode::INTERNAL_SERVER_ERROR, COOKIE_GET_ERROR).into_response();
        }
    };
    info_builder.obj_id(project_id);

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

// POST /api/projects/query
pub async fn query(
    State(state): State<AppState>,
    jar: CookieJar,
    URLQuery(params): URLQuery<HashMap<String, String>>,
    Json(details): Json<ProjectQueryRequest>,
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
                "Unable to query project from database",
            )
                .into_response();
        }
    };

    let project_models: Vec<ProjectModel> =
        rows.iter().map(|r| ProjectModel::from(r.clone())).collect();

    Json(project_models).into_response()
}
