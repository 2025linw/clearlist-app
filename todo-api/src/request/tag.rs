use serde::Deserialize;

use hex_color::HexColor;
use uuid::Uuid;

use crate::{
    database::TagModel,
    request::{
        api::{Create, Delete, Query, Retrieve, Update},
        query::CmpFlag,
        UpdateMethod,
    },
    response::Error,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TagPostRequest {
    label: Option<String>,
    category: Option<String>,

    color: Option<HexColor>,

    user_id: Option<Uuid>,
}

impl TagPostRequest {
    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[derive(Debug)]
pub struct TagGetRequest {
    tag_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl TagGetRequest {
    pub fn new() -> Self {
        Self {
            tag_id: None,

            user_id: None,
        }
    }

    pub fn tag_id(&mut self, id: Uuid) -> &mut Self {
        self.tag_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TagPutRequest {
    tag_id: Option<Uuid>,
    label: Option<String>,
    category: Option<String>,

    color: Option<HexColor>,

    user_id: Option<Uuid>,
}

impl TagPutRequest {
    pub fn tag_id(&mut self, id: Uuid) -> &mut Self {
        self.tag_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[derive(Debug)]
pub struct TagDeleteRequest {
    tag_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl TagDeleteRequest {
    pub fn new() -> Self {
        Self {
            tag_id: None,

            user_id: None,
        }
    }

    pub fn tag_id(&mut self, id: Uuid) -> &mut Self {
        self.tag_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TagQueryRequest {
    search_query: Option<String>,

    category: Option<String>,

    user_id: Option<Uuid>,
}

impl TagQueryRequest {
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
