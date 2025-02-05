use deadpool_postgres::Object;
use serde::Deserialize;
use tokio_postgres::{types::ToSql, Row};

use uuid::Uuid;

use crate::{
    database::AreaModel,
    request::{
        api::{Create, Delete, Info, Query, Retrieve, Update},
        query::CmpFlag,
        TIMESTAMP_NULL,
    },
    response::Error,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AreaPostRequest {
    name: Option<String>,

    icon_url: Option<String>,
}

#[derive(Debug)]
pub struct AreaGetRequest {}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AreaPutRequest {
    name: Option<String>,

    icon_url: Option<String>,
}

#[derive(Debug)]
pub struct AreaDeleteRequest {}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AreaQueryRequest {}
#[cfg(test)]
mod tests {
    // TODO: Task request testing
    use super::*;

    #[test]
    fn test_add() {
        assert!(true);
    }
}
