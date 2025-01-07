use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Input Model
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DateFilter {
    start_date: Option<NaiveDate>,
    start_cmp: Option<char>,

    deadline: Option<NaiveDate>,
    deadline_cmp: Option<char>,

    and: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskQueryModel {
    date_filter: Option<DateFilter>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectQueryModel {
    data_filter: Option<DateFilter>,

    area_id: Option<Uuid>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,
}

#[derive(Deserialize)]
pub struct TaskUpdateModel {
    title: Option<String>,
    notes: Option<String>,

    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    project_id: Option<Uuid>,
    area_id: Option<Uuid>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,
}

#[derive(Deserialize)]
pub struct ProjectUpdateModel {
    title: Option<String>,
    notes: Option<String>,

    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,
}

#[derive(Deserialize)]
pub struct AreaUpdateModel {
    name: Option<String>,

    icon_url: Option<String>,
}

#[derive(Deserialize)]
pub struct TagUpdateModel {
    label: Option<String>,
    category: Option<String>,

    color: Option<String>,
}

// Input/Output Model
#[derive(Deserialize, Serialize)]
pub struct TaskModel {
    id: Uuid,
    title: Option<String>,
    notes: Option<String>,

    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    project_id: Option<Uuid>,
    area_id: Option<Uuid>,

    created_on: NaiveDateTime,
    completed_on: Option<NaiveDateTime>,
    logged_on: Option<NaiveDateTime>,
    trashed_on: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct ProjectModel {
    id: Uuid,
    title: Option<String>,
    notes: Option<String>,

    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,

    created_on: NaiveDateTime,
    completed_on: Option<NaiveDateTime>,
    logged_on: Option<NaiveDateTime>,
    trashed_on: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct AreaModel {
    id: Uuid,
    name: Option<String>,

    icon_url: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TagModel {
    id: Uuid,
    label: Option<String>,
    category: Option<String>,

    color: Option<String>,
}
