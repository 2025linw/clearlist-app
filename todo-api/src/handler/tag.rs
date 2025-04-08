use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use serde_json::json;
use uuid::Uuid;

use crate::{
    AppState,
    schema::{
        FilterOptions,
        tag::{CreateTagSchema, QueryTagSchema, UpdateTagSchema},
    },
};

pub async fn create_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Json(body): Json<CreateTagSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO

    let json_message = json!({
        "status": "not implemented",
        "message": "handler has not been implemented",
    });

    return Err((StatusCode::NOT_IMPLEMENTED, Json(json_message)));

    let json_message = json!({
        "status": "ok",
        "data": "successful",
    });

    Ok(Json(json_message))
}

pub async fn retrieve_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO

    let json_message = json!({
        "status": "not implemented",
        "message": "handler has not been implemented",
    });

    return Err((StatusCode::NOT_IMPLEMENTED, Json(json_message)));

    let json_message = json!({
        "status": "ok",
        "data": "successful",
    });

    Ok(Json(json_message))
}

pub async fn update_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTagSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO

    let json_message = json!({
        "status": "not implemented",
        "message": "handler has not been implemented",
    });

    return Err((StatusCode::NOT_IMPLEMENTED, Json(json_message)));

    let json_message = json!({
        "status": "ok",
        "data": "successful",
    });

    Ok(Json(json_message))
}

pub async fn delete_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO

    let json_message = json!({
        "status": "not implemented",
        "message": "handler has not been implemented",
    });

    return Err((StatusCode::NOT_IMPLEMENTED, Json(json_message)));

    let json_message = json!({
        "status": "ok",
        "data": "successful",
    });

    Ok(Json(json_message))
}

pub async fn query_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Query(query): Query<FilterOptions>,
    Json(body): Json<QueryTagSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO

    let json_message = json!({
        "status": "not implemented",
        "message": "handler has not been implemented",
    });

    return Err((StatusCode::NOT_IMPLEMENTED, Json(json_message)));

    let json_message = json!({
        "status": "ok",
        "data": "successful",
    });

    Ok(Json(json_message))
}
