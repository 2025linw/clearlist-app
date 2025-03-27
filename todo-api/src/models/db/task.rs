use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::storage::db::DBModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskModel {
    task_id: Uuid,

    task_title: Option<String>,
    task_notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    completed_on: Option<DateTime<Local>>,
    logged_on: Option<DateTime<Local>>,
    trashed_on: Option<DateTime<Local>>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl TaskModel {
    pub const TABLE: &str = "data.tasks";

    pub const ID: &str = "task_id";

    pub const TITLE: &str = "task_title";
    pub const NOTES: &str = "task_notes";
    pub const START_DATE: &str = "start_date";
    pub const START_TIME: &str = "start_time";
    pub const DEADLINE: &str = "deadline";

    pub const COMPLETED: &str = "completed_on";
    pub const LOGGED: &str = "logged_on";
    pub const TRASHED: &str = "trashed_on";

    pub const AREA_ID: &str = "area_id";
    pub const PROJECT_ID: &str = "project_id";

    pub const USER_ID: &str = "user_id";
    pub const CREATED: &str = "created_on";
    pub const UPDATED: &str = "updated_on";
}

impl From<Row> for TaskModel {
    fn from(value: Row) -> Self {
        Self {
            task_id: value.get(TaskModel::ID),

            task_title: value.try_get(TaskModel::TITLE).ok(),
            task_notes: value.try_get(TaskModel::NOTES).ok(),
            start_date: value.try_get(TaskModel::START_DATE).ok(),
            start_time: value.try_get(TaskModel::START_TIME).ok(),
            deadline: value.try_get(TaskModel::DEADLINE).ok(),

            completed_on: value.try_get(TaskModel::COMPLETED).ok(),
            logged_on: value.try_get(TaskModel::LOGGED).ok(),
            trashed_on: value.try_get(TaskModel::TRASHED).ok(),

            area_id: value.try_get(TaskModel::AREA_ID).ok(),
            project_id: value.try_get(TaskModel::PROJECT_ID).ok(),

            user_id: value.get(TaskModel::USER_ID),
            created_on: value.get(TaskModel::CREATED),
            updated_on: value.get(TaskModel::UPDATED),
        }
    }
}

impl DBModel for TaskModel {}

#[derive(Debug)]
pub struct TaskTagModel {
    task_id: Uuid,
    tag_id: Uuid,
}

impl TaskTagModel {
    pub const TABLE: &str = "data.task_tags";

    pub const TASK_ID: &str = "task_id";
    pub const TAG_ID: &str = "tag_id";
}

impl From<Row> for TaskTagModel {
    fn from(value: Row) -> Self {
        Self {
            task_id: value.get(TaskTagModel::TASK_ID),
            tag_id: value.get(TaskTagModel::TAG_ID),
        }
    }
}

impl DBModel for TaskTagModel {}
