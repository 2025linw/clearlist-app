use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

pub const LOGIN_EXISTS: &str = "email already registered, login with password or reset password";
pub const LOGIN_AUTH: &str = "no user with email and password combination found";

pub const INTERNAL: &str = "unexpected error occured internally";
pub const INTERNAL_DB: &str = "unexpected error occured retrieving data";

pub const USER_REQUEST: &str = "invalid user request";

/// Error used with crate utility functions
///
/// Most function (except Axum handlers) will utilize this Error.
/// These errors are handled within each handler, however, not returned to client
#[derive(Debug)]
pub enum Error {
    InternalDatabase(String),
    Internal(String),
    UserRequest(String),
    UserAuth(String),
}

impl From<deadpool_postgres::PoolError> for Error {
    fn from(value: deadpool_postgres::PoolError) -> Self {
        Self::InternalDatabase(value.to_string())
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::InternalDatabase(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalDatabase(s)
            | Error::Internal(s)
            | Error::UserRequest(s)
            | Error::UserAuth(s) => write!(f, "Error: {}", s),
        }
    }
}

/// Error response for Axum handlers
///
/// These errors are returned back to the client
///
/// The error type is not used with any internal function except for handlers
pub struct ErrorResponse {
    code: StatusCode,
    msg: String,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, msg: &str) -> Self {
        Self {
            code,
            msg: msg.to_string(),
        }
    }
}

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        match value {
            Error::InternalDatabase(s) => {
                error!("Database error: {}", s);

                Self {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    msg: INTERNAL_DB.to_string(),
                }
            }
            Error::Internal(s) => {
                error!("Internal error: {}", s);

                Self {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    msg: INTERNAL.to_string(),
                }
            }
            Error::UserRequest(_) => Self {
                code: StatusCode::BAD_REQUEST,
                msg: USER_REQUEST.to_string(),
            },
            Error::UserAuth(s) => Self {
                code: StatusCode::UNAUTHORIZED,
                msg: s,
            },
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.code, self.msg).into_response()
    }
}
