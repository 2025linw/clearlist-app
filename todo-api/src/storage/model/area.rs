use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::storage::db::DBModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct AreaModel {
    area_id: Uuid,

    area_name: Option<String>,
    icon_url: Option<String>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl AreaModel {
    pub const TABLE: &str = "data.areas";

    pub const ID: &str = "area_id";

    pub const NAME: &str = "area_name";
    pub const ICON_URL: &str = "icon_url";

    pub const USER_ID: &str = "user_id";
    pub const CREATED: &str = "created_on";
    pub const UPDATED: &str = "updated_on";
}

impl From<Row> for AreaModel {
    fn from(value: Row) -> Self {
        Self {
            area_id: value.get(AreaModel::ID),

            area_name: value.try_get(AreaModel::NAME).ok(),
            icon_url: value.try_get(AreaModel::ICON_URL).ok(),

            user_id: value.get(AreaModel::USER_ID),
            created_on: value.get(AreaModel::CREATED),
            updated_on: value.get(AreaModel::UPDATED),
        }
    }
}

impl DBModel for AreaModel {}
