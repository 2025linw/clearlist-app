use serde::Deserialize;

use super::{QueryMethod, UpdateMethod};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagSchema {
    label: Option<String>,
    category: Option<String>,
    color: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagSchema {
    label: Option<UpdateMethod<String>>,
    category: Option<UpdateMethod<String>>,
    color: Option<UpdateMethod<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTagSchema {
    label: Option<QueryMethod<String>>,
    category: Option<QueryMethod<String>>,
    color: Option<QueryMethod<String>>,
}
