use tokio_postgres::Row;
use uuid::Uuid;

/// Task tags database model
pub struct DatabaseModel {
    task_id: Uuid,
    tag_id: Uuid,
}

impl DatabaseModel {
    pub const TABLE: &str = "data.task_tags";

    pub const TASK_ID: &str = "task_id";
    pub const TAG_ID: &str = "tag_id";
}

impl From<Row> for DatabaseModel {
    fn from(value: Row) -> Self {
        Self {
            task_id: value.get(Self::TASK_ID),
            tag_id: value.get(Self::TAG_ID),
        }
    }
}

pub struct CreateSchema {
    task_id: Uuid,
    tag_id: Uuid,
}
