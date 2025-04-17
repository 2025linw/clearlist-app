use axum::{Json, http::StatusCode};
use serde_json::json;

// TODO: use thiserror
// TODO: better differentiate between different types of errors (user auth, database query, database pool, etc)
// TODO: include server side console prints or logs
// TODO: include client side responses that don't reveal too much about server architecture

/// Errors that are used to return back to Client
///
/// Any function that returns to client must use this to avoid
/// revealing any information about backend issues.
#[derive(Debug)]
pub enum Error {
    DatabasePool(deadpool_postgres::PoolError),
    Database(tokio_postgres::Error),
    InvalidRequest(String),
    Internal,
    Generic,
}

impl Error {
    pub fn to_axum_response(&self) -> (StatusCode, Json<serde_json::Value>) {
        match self {
            Error::DatabasePool(pool_error) => {
                eprintln!("Database Pool Error: {:#}", pool_error);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": "database connection error",
                    })),
                )
            }
            Error::Database(error) => {
                eprintln!("Database Error: {:#}", error);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": "database query error"
                    })),
                )
            }
            Error::InvalidRequest(s) => {
                eprintln!("Error: {:#}", s);

                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "status": "error",
                        "message": "error in the request"
                    })),
                )
            }
            Error::Internal => {
                eprintln!("Internal Error");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": "internal error occured",
                    })),
                )
            }
            Error::Generic => {
                eprintln!("Generic Error");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": "generic error occured"
                    })),
                )
            }
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
