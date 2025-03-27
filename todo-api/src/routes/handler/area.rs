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
        db::AreaModel,
        request::{
            AreaCreateRequest, AreaDeleteRequest, AreaQueryRequest, AreaRetrieveRequest,
            AreaUpdateRequest,
        },
    },
    routes::utils::extract_user_id,
    storage::db,
};

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
    Query(params): Query<HashMap<String, String>>,
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
