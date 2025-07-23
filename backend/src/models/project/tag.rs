use tokio_postgres::Row;
use uuid::Uuid;

/// Project tags database model
pub struct DatabaseModel {
    project_id: Uuid,
    tag_id: Uuid,
}

impl DatabaseModel {
    pub const TABLE: &str = "data.project_tags";

    pub const PROJECT_ID: &str = "project_id";
    pub const TAG_ID: &str = "tag_id";
}

impl From<Row> for DatabaseModel {
    fn from(value: Row) -> Self {
        Self {
            project_id: value.get(Self::PROJECT_ID),
            tag_id: value.get(Self::TAG_ID),
        }
    }
}

// TODO: create models for retrieving project-tag relations
// Should we make a query that returns just the relation or the full tag that relates to the project?
// Retrieve

// Update

// Delete
