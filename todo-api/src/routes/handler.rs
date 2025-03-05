use axum::{
    Json,
    extract::{Extension, Path, Query as URLQuery},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use deadpool_postgres::Object;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use crate::{
    AppState,
    storage::{db, model::*},
};

use super::{extract_user_id, model::*};

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
    URLQuery(params): URLQuery<HashMap<String, String>>,
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
    URLQuery(params): URLQuery<HashMap<String, String>>,
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

// POST /api/areas
pub async fn create_area(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Json(details): Json<AreaCreateRequest>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: AreaCreateRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };

    let area_id: Uuid = match db::insert(&mut conn, &details).await {
        Ok(i) => i,
        Err(e) => return e.into_response(),
    };

    (StatusCode::CREATED, area_id.to_string()).into_response()
}

// GET /api/areas/:id
pub async fn retrieve_area(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(area_id): Path<uuid::Uuid>,
) -> Response {
    let conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: AreaRetrieveRequest = AreaRetrieveRequest::default();
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_area_id(area_id);

    let row_opt: Option<AreaModel> = match db::select_one(&conn, &details).await {
        Ok(o) => o,
        Err(e) => return e.into_response(),
    };

    match row_opt {
        Some(r) => Json(r).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// PUT /api/areas/:id
pub async fn update_area(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(area_id): Path<uuid::Uuid>,
    Json(details): Json<AreaUpdateRequest>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: AreaUpdateRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_area_id(area_id);

    let row_opt: Option<AreaModel> = match db::update(&mut conn, &details).await {
        Ok(o) => o,
        Err(e) => return e.into_response(),
    };

    match row_opt {
        Some(r) => Json(r).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// DELETE /api/areas/:id
pub async fn delete_area(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(area_id): Path<uuid::Uuid>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: AreaDeleteRequest = AreaDeleteRequest::default();
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_area_id(area_id);

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

// POST /api/areas/query
pub async fn query_areas(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    URLQuery(params): URLQuery<HashMap<String, String>>,
    Json(details): Json<AreaQueryRequest>,
) -> Response {
    let conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: AreaQueryRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    if let Some(query) = params.get("query") {
        details.set_search_query(query.to_owned());
    }

    let area_models: Vec<AreaModel> = match db::select_all(&conn, &details).await {
        Ok(v) => v,
        Err(e) => return e.into_response(),
    };

    Json(area_models).into_response()
}

// POST /api/tags
pub async fn create_tag(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Json(details): Json<TagCreateRequest>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TagCreateRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };

    let tag_id: Uuid = match db::insert(&mut conn, &details).await {
        Ok(i) => i,
        Err(e) => return e.into_response(),
    };

    (StatusCode::CREATED, tag_id.to_string()).into_response()
}

// GET /api/tags/:id
pub async fn retrieve_tag(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(tag_id): Path<uuid::Uuid>,
) -> Response {
    let conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TagRetrieveRequest = TagRetrieveRequest::default();
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_tag_id(tag_id);

    let row_opt: Option<TagModel> = match db::select_one(&conn, &details).await {
        Ok(o) => o,
        Err(e) => return e.into_response(),
    };

    match row_opt {
        Some(r) => Json(r).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// PUT /api/tags/:id
pub async fn update_tag(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(tag_id): Path<uuid::Uuid>,
    Json(details): Json<TagUpdateRequest>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TagUpdateRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_tag_id(tag_id);

    let row_opt: Option<TagModel> = match db::update(&mut conn, &details).await {
        Ok(r) => r,
        Err(e) => return e.into_response(),
    };

    match row_opt {
        Some(r) => Json(r).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// DELETE /api/tags/:id
pub async fn delete_tag(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(tag_id): Path<uuid::Uuid>,
) -> Response {
    let mut conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TagDeleteRequest = TagDeleteRequest::default();
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    details.set_tag_id(tag_id);

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

// POST /api/tags/query
pub async fn query_tags(
    Extension(state): Extension<Arc<AppState>>,
    jar: CookieJar,
    URLQuery(params): URLQuery<HashMap<String, String>>,
    Json(details): Json<TagQueryRequest>,
) -> Response {
    let conn: Object = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let mut details: TagQueryRequest = details;
    match extract_user_id(&jar) {
        Ok(i) => details.set_user_id(i),
        Err(e) => return e.into_response(),
    };
    if let Some(query) = params.get("query") {
        details.set_search_query(query.to_owned());
    }

    let tag_models: Vec<TagModel> = match db::select_all(&conn, &details).await {
        Ok(v) => v,
        Err(e) => return e.into_response(),
    };

    Json(tag_models).into_response()
}
