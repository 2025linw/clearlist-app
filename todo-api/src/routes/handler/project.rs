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
        db::ProjectModel,
        request::{
            ProjectCreateRequest, ProjectDeleteRequest, ProjectQueryRequest,
            ProjectRetrieveRequest, ProjectUpdateRequest,
        },
    },
    routes::utils::extract_user_id,
    storage::db,
};

// POST /api/projects
pub async fn create_project(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Json(details): Json<ProjectCreateRequest>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: ProjectCreateRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };

    let project_id: Uuid = match db::insert(&mut conn, &details).await {
        Ok(i) => i,
        Err(e) => return e.into_response(),
    };

    (StatusCode::CREATED, project_id.to_string()).into_response()
}

// GET /api/projects/:id
pub async fn retrieve_project(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(project_id): Path<uuid::Uuid>,
) -> Response {
    let conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: ProjectRetrieveRequest = ProjectRetrieveRequest::default();
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_project_id(project_id);

    let row_opt: Option<ProjectModel> = match db::select_one(&conn, &details).await {
        Ok(o) => o,
        Err(e) => return e.into_response(),
    };

    match row_opt {
        Some(r) => Json(r).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// PUT /api/projects/:id
pub async fn update_project(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(project_id): Path<uuid::Uuid>,
    Json(details): Json<ProjectUpdateRequest>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: ProjectUpdateRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_project_id(project_id);

    let row_opt: Option<ProjectModel> = match db::update(&mut conn, &details).await {
        Ok(o) => o,
        Err(e) => return e.into_response(),
    };

    match row_opt {
        Some(r) => Json(r).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// DELETE /api/projects/:id
pub async fn delete_project(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(project_id): Path<uuid::Uuid>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: ProjectDeleteRequest = ProjectDeleteRequest::default();
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_project_id(project_id);

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

// POST /api/projects/query
pub async fn query_projects(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Query(params): Query<HashMap<String, String>>,
    Json(details): Json<ProjectQueryRequest>,
) -> Response {
    let conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: ProjectQueryRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    if let Some(query) = params.get("query") {
        details.set_search_query(query.to_owned());
    }

    let project_models: Vec<ProjectModel> = match db::select_all(&conn, &details).await {
        Ok(v) => v,
        Err(e) => return e.into_response(),
    };

    Json(project_models).into_response()
}
