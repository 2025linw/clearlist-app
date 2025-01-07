use axum::extract::{Json, Path, State};

use super::AppState;
use crate::models;

// POST /api/areas
pub async fn create_area(State(state): State<AppState>, Json(details): Json<models::TaskModel>) {
    todo!()
}

// GET /api/areas/:id
pub async fn retrieve_area(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) {
    todo!()
}

// PUT /api/areas/:id
pub async fn update_area(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(details): Json<models::AreaUpdateModel>,
) {
    todo!()
}

// DELETE /api/areas/:id
pub async fn delete_area(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) {
    todo!()
}
