use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// TODO: use thiserror

pub enum Error {
    DatabasePool(deadpool_postgres::PoolError),
    Database(tokio_postgres::Error),
    InvalidRequest(String),
    Internal,
    Generic,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::DatabasePool(s) => {
                (StatusCode::INTERNAL_SERVER_ERROR, s.to_string()).into_response()
            }
            Self::Database(s) => (StatusCode::INTERNAL_SERVER_ERROR, s.to_string()).into_response(),
            Self::InvalidRequest(s) => (StatusCode::BAD_REQUEST, s).into_response(),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            Self::Generic => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

impl From<deadpool_postgres::PoolError> for Error {
    fn from(value: deadpool_postgres::PoolError) -> Self {
        Self::DatabasePool(value)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::Database(value)
    }
}
