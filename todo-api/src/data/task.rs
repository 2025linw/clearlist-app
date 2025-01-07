use axum::extract::{Json, Path, Query, State};
use std::collections::HashMap;

use super::AppState;
use crate::models;

// POST /api/tasks
pub async fn create_task(State(state): State<AppState>, Json(details): Json<models::TaskModel>) {
    todo!()
}

// GET /api/tasks/:id
pub async fn retrieve_task(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) -> String {
    todo!()
}

// PUT /api/tasks/:id
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(details): Json<models::TaskUpdateModel>,
) {
    todo!()
}

// DELETE /api/tasks/:id
pub async fn delete_task(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) {
    todo!()
}

// POST /api/tasks/query
pub async fn query_tasks(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    Json(details): Json<models::TaskQueryModel>,
) -> String {
    todo!()
}
