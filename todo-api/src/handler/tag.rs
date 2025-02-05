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
    database::TagModel,
    database::AppState,
    request::{
        api::{Create, Delete, InfoBuilder, Query, Retrieve, Update},
        extract_user_id,
        tag::*,
    },
    response::{COOKIE_GET_ERROR, SERVER_POOL_ERROR},
};

// Database Model

// POST /api/tags
pub async fn create(State(state): State<AppState>, Json(details): Json<TagModel>) -> Response {
    let state = state.clone();
    let conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("{err}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    todo!("Create Tag")
}

// GET /api/tags/:id
pub async fn retrieve(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) -> Response {
    let state = state.clone();
    let conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("{err}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    todo!("Retrieve Tag")
}

// PUT /api/tags/:id
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(details): Json<TagUpdateRequest>,
) -> Response {
    let state = state.clone();
    let conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("{err}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    todo!("Update Tag")
}

// DELETE /api/tags/:id
pub async fn delete(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) -> Response {
    let state = state.clone();
    let conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("{err}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    todo!("Delete Tag")
}

// POST /api/tags/query
pub async fn query(
    State(state): State<AppState>,
    URLQuery(params): URLQuery<HashMap<String, String>>,
    Json(details): Json<TagQueryRequest>,
) -> Response {
    todo!("Query Tags")
}
