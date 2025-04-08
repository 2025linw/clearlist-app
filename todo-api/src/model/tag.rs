use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use super::ToResponse;

/// Database Model
#[derive(Debug, Deserialize)]
pub struct TagModel {
    tag_id: Uuid,

    tag_label: Option<String>,
    tag_category: Option<String>,
    tag_color: Option<String>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl TagModel {
    pub const TABLE: &str = "data.tags";

    pub const ID: &str = "tag_id";

    pub const LABEL: &str = "tag_label";
    pub const CATEGORY: &str = "tag_category";
    pub const COLOR: &str = "tag_color";

    pub const USER_ID: &str = "user_id";
    pub const CREATED: &str = "created_on";
    pub const UPDATED: &str = "updated_on";
}

impl From<Row> for TagModel {
    fn from(value: Row) -> Self {
        Self {
            tag_id: value.get(Self::ID),
            tag_label: value.get(Self::LABEL),
            tag_category: value.get(Self::CATEGORY),
            tag_color: value.get(Self::COLOR),
            user_id: value.get(Self::USER_ID),
            created_on: value.get(Self::CREATED),
            updated_on: value.get(Self::UPDATED),
        }
    }
}

impl ToResponse for TagModel {
    type Response = TagModelResponse;

    fn to_response(&self) -> Self::Response {
        Self::Response {
            id: self.tag_id,

            label: self.tag_label.to_owned().unwrap_or_default(),
            category: self.tag_category.to_owned().unwrap_or_default(),
            color: self.tag_color.to_owned().unwrap_or_default(),

            user_id: self.user_id,
            created_on: self.created_on,
            updated_on: self.updated_on,
        }
    }
}

/// Response Model
#[derive(Debug, Deserialize, Serialize)]
pub struct TagModelResponse {
    id: Uuid,

    label: String,
    category: String,
    color: String,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}
