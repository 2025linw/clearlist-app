use std::fmt::Display;

pub const SERVER_POOL_ERROR: &str = "Unable to get connection to database from pool";
pub const UUID_PARSE_ERROR: &str = "Unable to parse UUID from string";
pub const COOKIE_GET_ERROR: &str = "Unable to retrieve cookie";

pub enum Error {
    QueryError(String),
    DatabaseError(tokio_postgres::Error),
}

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::DatabaseError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryError(s) => write!(f, "{}", s),
            Self::DatabaseError(e) => write!(f, "{}", e),
        }
    }
}
