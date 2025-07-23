use axum::{http::StatusCode, response::IntoResponse};

use crate::error::ErrorResponse;

pub async fn retrieve_handler() -> Result<impl IntoResponse, ErrorResponse> {
    // TODO

    Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn update_handler() -> Result<impl IntoResponse, ErrorResponse> {
    // TODO

    Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete_handler() -> Result<impl IntoResponse, ErrorResponse> {
    // TODO

    Ok(StatusCode::NOT_IMPLEMENTED)
}
