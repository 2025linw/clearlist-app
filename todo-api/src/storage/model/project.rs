use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::storage::db::DBModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectModel {
    project_id: Uuid,

    project_title: Option<String>,
    project_notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    completed_on: Option<DateTime<Local>>,
    logged_on: Option<DateTime<Local>>,
    trashed_on: Option<DateTime<Local>>,

    area_id: Option<Uuid>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl ProjectModel {
    pub const TABLE: &str = "data.projects";

    pub const ID: &str = "project_id";

    pub const TITLE: &str = "project_title";
    pub const NOTES: &str = "project_notes";
    pub const START_DATE: &str = "start_date";
    pub const START_TIME: &str = "start_time";
    pub const DEADLINE: &str = "deadline";

    pub const COMPLETED: &str = "completed_on";
    pub const LOGGED: &str = "logged_on";
    pub const TRASHED: &str = "trashed_on";

    pub const AREA_ID: &str = "area_id";

    pub const USER_ID: &str = "user_id";
    pub const CREATED: &str = "created_on";
    pub const UPDATED: &str = "updated_on";
}

impl From<Row> for ProjectModel {
    fn from(value: Row) -> Self {
        Self {
            project_id: value.get(ProjectModel::ID),

            project_title: value.try_get(ProjectModel::TITLE).ok(),
            project_notes: value.try_get(ProjectModel::NOTES).ok(),
            start_date: value.try_get(ProjectModel::START_DATE).ok(),
            start_time: value.try_get(ProjectModel::START_TIME).ok(),
            deadline: value.try_get(ProjectModel::DEADLINE).ok(),
            completed_on: value.try_get(ProjectModel::COMPLETED).ok(),
            logged_on: value.try_get(ProjectModel::LOGGED).ok(),
            trashed_on: value.try_get(ProjectModel::TRASHED).ok(),

            area_id: value.try_get(ProjectModel::AREA_ID).ok(),

            user_id: value.get(ProjectModel::USER_ID),
            created_on: value.get(ProjectModel::CREATED),
            updated_on: value.get(ProjectModel::UPDATED),
        }
    }
}

impl DBModel for ProjectModel {}

pub struct ProjectTagModel {
    project_id: Uuid,
    tag_id: Uuid,
}

impl ProjectTagModel {
    pub const TABLE: &str = "data.project_tags";

    pub const PROJECT_ID: &str = "project_id";
    pub const TAG_ID: &str = "tag_id";
}

impl From<Row> for ProjectTagModel {
    fn from(value: Row) -> Self {
        Self {
            project_id: value.get(ProjectTagModel::PROJECT_ID),
            tag_id: value.get(ProjectTagModel::TAG_ID),
        }
    }
}

impl DBModel for ProjectTagModel {}
