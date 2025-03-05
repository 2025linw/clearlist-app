use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::storage::db::DBModel;

#[derive(Debug, Deserialize, Serialize)]
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
            tag_id: value.get(TagModel::ID),

            tag_label: value.try_get(TagModel::LABEL).ok(),
            tag_category: value.try_get(TagModel::CATEGORY).ok(),
            tag_color: value.try_get(TagModel::COLOR).ok(),

            user_id: value.get(TagModel::USER_ID),
            created_on: value.get(TagModel::CREATED),
            updated_on: value.get(TagModel::UPDATED),
        }
    }
}

impl DBModel for TagModel {}
