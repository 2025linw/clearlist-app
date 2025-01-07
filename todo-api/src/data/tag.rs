use axum::extract::{Json, Path, State};

use super::AppState;
use crate::models;

// POST /api/tags
pub async fn create_tag(State(state): State<AppState>, Json(details): Json<models::TagModel>) {
    todo!()
}

// GET /api/tags/:id
pub async fn retrieve_tag(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) {
    todo!()
}

// PUT /api/tags/:id
pub async fn update_tag(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(details): Json<models::TagUpdateModel>,
) {
    todo!()
}

// DELETE /api/tags/:id
pub async fn delete_tag(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) {
    todo!()
}
