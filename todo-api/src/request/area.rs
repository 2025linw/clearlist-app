use deadpool_postgres::Object;
use serde::Deserialize;
use tokio_postgres::{types::ToSql, Row};

use uuid::Uuid;

use crate::{
    database::AreaModel,
    request::{
        api::{Create, Delete, Query, Retrieve, Update},
        query::CmpFlag,
    },
    response::Error,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AreaPostRequest {
    name: Option<String>,

    icon_url: Option<String>,

    user_id: Option<Uuid>,
}

impl AreaPostRequest {
    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[derive(Debug)]
pub struct AreaGetRequest {
    area_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl AreaGetRequest {
    pub fn new() -> Self {
        Self {
            area_id: None,

            user_id: None,
        }
    }

    pub fn area_id(&mut self, id: Uuid) -> &mut Self {
        self.area_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AreaPutRequest {
    area_id: Option<Uuid>,
    name: Option<String>,

    icon_url: Option<String>,

    user_id: Option<Uuid>,
}

impl AreaPutRequest {
    pub fn area_id(&mut self, id: Uuid) -> &mut Self {
        self.area_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[derive(Debug)]
pub struct AreaDeleteRequest {
    area_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl AreaDeleteRequest {
    pub fn new() -> Self {
        Self {
            area_id: None,

            user_id: None,
        }
    }

    pub fn area_id(&mut self, id: Uuid) -> &mut Self {
        self.area_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AreaQueryRequest {
    search_query: Option<String>,

    user_id: Option<Uuid>,
}

impl AreaQueryRequest {
    pub fn search_query(&mut self, query: String) -> &mut Self {
        self.search_query = Some(query);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[cfg(test)]
mod tests {
    // TODO: Task request testing
    use super::*;

    #[test]
    fn test_add() {
        assert!(true);
    }
}
