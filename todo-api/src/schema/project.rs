use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;
use uuid::Uuid;

use super::{QueryMethod, UpdateMethod};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectSchema {
    title: Option<String>,
    notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectSchema {
    title: Option<UpdateMethod<String>>,
    notes: Option<UpdateMethod<String>>,
    start_date: Option<UpdateMethod<NaiveDate>>,
    start_time: Option<UpdateMethod<NaiveTime>>,
    deadline: Option<UpdateMethod<NaiveDate>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    area_id: Option<UpdateMethod<Uuid>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryProjectSchema {
    title: Option<QueryMethod<String>>,
    notes: Option<QueryMethod<String>>,
    start_date: Option<QueryMethod<NaiveDate>>,
    start_time: Option<QueryMethod<NaiveTime>>,
    deadline: Option<QueryMethod<NaiveDate>>,

    completed: Option<QueryMethod<NaiveDate>>,
    logged: Option<QueryMethod<NaiveTime>>,
    trashed: Option<QueryMethod<NaiveDate>>,

    area_id: Option<QueryMethod<Uuid>>,
}
