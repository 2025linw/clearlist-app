use serde::Deserialize;

use hex_color::HexColor;

use crate::{
	database::{TagModel},
	request::{
		api::{Create, Delete, Info, Query, Retrieve, Update},
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
}

#[derive(Debug)]
pub struct TagGetRequest {}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TagUpdateRequest {
    label: Option<String>,
    category: Option<String>,

    color: Option<HexColor>,
}

#[derive(Debug)]
pub struct TagDeleteRequest {}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TagQueryRequest {
    category: Option<String>,
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
