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
        db::TagModel,
        request::{
            TagCreateRequest, TagDeleteRequest, TagQueryRequest, TagRetrieveRequest,
            TagUpdateRequest,
        },
    },
    routes::utils::extract_user_id,
    storage::db,
};

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
    Query(params): Query<HashMap<String, String>>,
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
