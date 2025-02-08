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
    database::AppState,
    database::AreaModel,
    request::{
        api::{extract_user_id, Create, Delete, Query, Retrieve, Update},
        area::*,
    },
    response::SERVER_POOL_ERROR,
};

// POST /api/areas
pub async fn create(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(details): Json<AreaModel>,
) -> Response {
    let state = state.clone();
    let conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    todo!("Create Area")
}

// GET /api/areas/:id
pub async fn retrieve(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    let state = state.clone();
    let conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    todo!("Retrieve Area")
}

// PUT /api/areas/:id
pub async fn update(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<uuid::Uuid>,
    Json(details): Json<AreaPutRequest>,
) -> Response {
    let state = state.clone();
    let conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    todo!("Update Area")
}

// DELETE /api/areas/:id
pub async fn delete(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    let state = state.clone();
    let conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");

            return (StatusCode::INTERNAL_SERVER_ERROR, SERVER_POOL_ERROR).into_response();
        }
    };

    todo!("Delete Area")
}

// POST /api/areas/query
pub async fn query(
    State(state): State<AppState>,
    jar: CookieJar,
    URLQuery(params): URLQuery<HashMap<String, String>>,
    Json(details): Json<AreaQueryRequest>,
) -> Response {
    todo!("Query Areas")
}
