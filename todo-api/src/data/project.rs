use axum::extract::{Json, Path, Query, State};
use std::collections::HashMap;

use super::AppState;
use crate::models;

// POST /api/projects
pub async fn create_project(
    State(state): State<AppState>,
    Json(details): Json<models::ProjectModel>,
) {
    todo!()
}

// GET /api/projects/:id
pub async fn retrieve_project(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) {
    todo!()
}

// PUT /api/projects/:id
pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(details): Json<models::ProjectUpdateModel>,
) {
    todo!()
}

// DELETE /api/projects/:id
pub async fn delete_project(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) {
    todo!()
}

// POST /api/projects/query
pub async fn query_projects(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    Json(details): Json<models::ProjectQueryModel>,
) {
    todo!()
}
