use serde::Deserialize;

use super::{QueryMethod, UpdateMethod};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAreaSchema {
    name: Option<String>,
    icon_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAreaSchema {
    name: Option<UpdateMethod<String>>,
    icon_url: Option<UpdateMethod<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryAreaSchema {
    name: Option<QueryMethod<String>>,
    icon_url: Option<QueryMethod<String>>,
}
