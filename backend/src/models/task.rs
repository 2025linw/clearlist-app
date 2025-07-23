pub mod tag;

use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::{
    models::tag as tag_model,
    util::{NULL, PostgresCmp, SQLQueryBuilder, ToPostgresCmp, ToSQLQueryBuilder},
};

use super::{QueryMethod, ToResponse, UpdateMethod};

/// Task Database Model
#[derive(Debug)]
pub struct DatabaseModel {
    id: Uuid,

    task_title: Option<String>,
    notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    completed_on: Option<DateTime<Local>>,
    logged_on: Option<DateTime<Local>>,
    trashed_on: Option<DateTime<Local>>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,
    tags: Vec<tag_model::DatabaseModel>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl DatabaseModel {
    pub fn id_as_ref(&self) -> &Uuid {
        &self.id
    }

    pub fn set_tags(&mut self, tags: Vec<tag_model::DatabaseModel>) {
        self.tags = tags;
    }
}

impl DatabaseModel {
    pub const TABLE: &str = "data.tasks";

    pub const ID: &str = "task_id";

    pub const TITLE: &str = "task_title";
    pub const NOTES: &str = "notes";
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

impl From<Row> for DatabaseModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get(Self::ID),
            task_title: value.get(Self::TITLE),
            notes: value.get(Self::NOTES),
            start_date: value.get(Self::START_DATE),
            start_time: value.get(Self::START_TIME),
            deadline: value.get(Self::DEADLINE),
            completed_on: value.get(Self::COMPLETED),
            logged_on: value.get(Self::LOGGED),
            trashed_on: value.get(Self::TRASHED),
            area_id: value.get(Self::AREA_ID),
            project_id: value.get(Self::PROJECT_ID),
            tags: Vec::new(),
            user_id: value.get(Self::USER_ID),
            created_on: value.get(Self::CREATED),
            updated_on: value.get(Self::UPDATED),
        }
    }
}

impl ToResponse for DatabaseModel {
    type Response = ResponseModel;

    fn to_response(&self) -> Self::Response {
        Self::Response {
            id: self.id,
            title: self.task_title.to_owned().unwrap_or_default(),
            notes: self.notes.to_owned().unwrap_or_default(),
            start_date: self.start_date,
            start_time: self.start_time,
            deadline: self.deadline,
            completed_on: self.completed_on,
            logged_on: self.logged_on,
            trashed_on: self.trashed_on,
            area_id: self.area_id,
            project_id: self.project_id,
            tags: self.tags.iter().map(|t| t.to_response()).collect(),
            user_id: self.user_id,
            created_on: self.created_on,
            updated_on: self.updated_on,
        }
    }
}

/// Task Response Model
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseModel {
    id: Uuid,

    title: String,
    notes: String,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    completed_on: Option<DateTime<Local>>,
    logged_on: Option<DateTime<Local>>,
    trashed_on: Option<DateTime<Local>>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,
    tags: Vec<tag_model::ResponseModel>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    title: Option<String>,
    notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,
    tag_ids: Vec<Uuid>,
}

impl CreateRequest {
    pub fn tag_ids(&self) -> &[Uuid] {
        &self.tag_ids
    }
}

impl ToSQLQueryBuilder for CreateRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.set_return(&[DatabaseModel::ID]);

        if let Some(ref s) = self.title {
            builder.add_column(DatabaseModel::TITLE, s);
        }

        if let Some(ref s) = self.notes {
            builder.add_column(DatabaseModel::NOTES, s);
        }
        if let Some(ref d) = self.start_date {
            builder.add_column(DatabaseModel::START_DATE, d);
        }
        if let Some(ref t) = self.start_time {
            builder.add_column(DatabaseModel::START_TIME, t);
        }
        if let Some(ref d) = self.deadline {
            builder.add_column(DatabaseModel::DEADLINE, d);
        }

        if let Some(ref i) = self.area_id {
            builder.add_column(DatabaseModel::AREA_ID, i);
        }
        if let Some(ref i) = self.project_id {
            builder.add_column(DatabaseModel::PROJECT_ID, i);
        }

        builder
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct UpdateRequest {
    title: UpdateMethod<String>,
    notes: UpdateMethod<String>,
    start_date: UpdateMethod<NaiveDate>,
    start_time: UpdateMethod<NaiveTime>,
    deadline: UpdateMethod<NaiveDate>,

    completed: UpdateMethod<bool>,
    logged: UpdateMethod<bool>,
    trashed: UpdateMethod<bool>,

    area_id: UpdateMethod<Uuid>,
    project_id: UpdateMethod<Uuid>,
    tag_ids: UpdateMethod<Vec<Uuid>>,

    #[serde(default = "chrono::Local::now")]
    timestamp: DateTime<Local>,
}

impl UpdateRequest {
    pub fn is_empty(&self) -> bool {
        self.title.is_noop()
            && self.notes.is_noop()
            && self.start_date.is_noop()
            && self.start_time.is_noop()
            && self.deadline.is_noop()
            && self.completed.is_noop()
            && self.logged.is_noop()
            && self.trashed.is_noop()
            && self.area_id.is_noop()
            && self.project_id.is_noop()
            && self.tag_ids.is_noop()
    }

    pub fn tag_ids(&self) -> Option<&[Uuid]> {
        if let UpdateMethod::Set(ref tag_ids) = self.tag_ids {
            Some(tag_ids)
        } else {
            None
        }
    }
}

impl ToSQLQueryBuilder for UpdateRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_column(DatabaseModel::UPDATED, &self.timestamp);
        builder.set_return(&[DatabaseModel::ID]);

        if !self.title.is_noop() {
            builder.add_column(DatabaseModel::TITLE, &self.title);
        }
        if !self.notes.is_noop() {
            builder.add_column(DatabaseModel::NOTES, &self.notes);
        }
        if !self.start_date.is_noop() {
            builder.add_column(DatabaseModel::START_DATE, &self.start_date);
        }
        if !self.start_time.is_noop() {
            builder.add_column(DatabaseModel::START_TIME, &self.start_time);
        }
        if !self.deadline.is_noop() {
            builder.add_column(DatabaseModel::DEADLINE, &self.deadline);
        }

        if !self.completed.is_noop() {
            match self.completed {
                UpdateMethod::Set(true) => {
                    builder.add_column(DatabaseModel::COMPLETED, &self.timestamp);
                }
                UpdateMethod::Set(false) | UpdateMethod::Remove => {
                    builder.add_column(DatabaseModel::COMPLETED, &None::<DateTime<Local>>);
                }
                UpdateMethod::NoOp => unreachable!(),
            };
        }
        if !self.logged.is_noop() {
            match self.logged {
                UpdateMethod::Set(true) => {
                    builder.add_column(DatabaseModel::LOGGED, &self.timestamp)
                }
                UpdateMethod::Set(false) | UpdateMethod::Remove => {
                    builder.add_column(DatabaseModel::LOGGED, &None::<DateTime<Local>>)
                }
                UpdateMethod::NoOp => unreachable!(),
            };
        }
        if !self.trashed.is_noop() {
            match self.trashed {
                UpdateMethod::Set(true) => {
                    builder.add_column(DatabaseModel::TRASHED, &self.timestamp)
                }
                UpdateMethod::Set(false) | UpdateMethod::Remove => {
                    builder.add_column(DatabaseModel::TRASHED, &None::<DateTime<Local>>)
                }
                UpdateMethod::NoOp => unreachable!(),
            };
        }

        if !self.area_id.is_noop() {
            builder.add_column(DatabaseModel::AREA_ID, &self.area_id);
        }
        if !self.project_id.is_noop() {
            builder.add_column(DatabaseModel::PROJECT_ID, &self.project_id);
        }

        builder
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    title: Option<QueryMethod<String>>,
    notes: Option<QueryMethod<String>>,
    start_date: Option<QueryMethod<NaiveDate>>,
    start_time: Option<QueryMethod<NaiveTime>>,
    deadline: Option<QueryMethod<NaiveDate>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,
    tag_ids: Vec<Uuid>,
}

impl QueryRequest {
    pub fn tag_ids(&self) -> &[Uuid] {
        &self.tag_ids
    }
}

impl ToSQLQueryBuilder for QueryRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.set_return_all();

        if let Some(ref q) = self.title {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::ILike,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(DatabaseModel::TITLE, cmp, q);
        }
        if let Some(ref q) = self.notes {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::ILike,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(DatabaseModel::NOTES, cmp, q);
        }
        if let Some(ref q) = self.start_date {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Equal,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(DatabaseModel::START_DATE, cmp, q);
        }
        if let Some(ref q) = self.start_time {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Equal,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(DatabaseModel::START_TIME, cmp, q);
        }
        if let Some(ref q) = self.deadline {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Equal,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(DatabaseModel::DEADLINE, cmp, q);
        }

        if let Some(b) = self.completed {
            if b {
                builder.add_condition(DatabaseModel::COMPLETED, PostgresCmp::NotNull, &NULL);
            } else {
                builder.add_condition(DatabaseModel::COMPLETED, PostgresCmp::IsNull, &NULL);
            }
        }
        if let Some(b) = self.logged {
            if b {
                builder.add_condition(DatabaseModel::LOGGED, PostgresCmp::NotNull, &NULL);
            } else {
                builder.add_condition(DatabaseModel::LOGGED, PostgresCmp::IsNull, &NULL);
            }
        }
        if let Some(b) = self.trashed {
            if b {
                builder.add_condition(DatabaseModel::TRASHED, PostgresCmp::NotNull, &NULL);
            } else {
                builder.add_condition(DatabaseModel::TRASHED, PostgresCmp::IsNull, &NULL);
            }
        }

        if let Some(ref i) = self.area_id {
            builder.add_condition(DatabaseModel::AREA_ID, PostgresCmp::Equal, i);
        }
        if let Some(ref i) = self.project_id {
            builder.add_condition(DatabaseModel::PROJECT_ID, PostgresCmp::Equal, i);
        }

        builder
    }
}

#[cfg(test)]
mod create_schema {
    use chrono::Local;
    use uuid::Uuid;

    use crate::util::ToSQLQueryBuilder;

    use super::CreateRequest;

    #[test]
    fn text_only() {
        let mut schema = CreateRequest::default();
        schema.title = Some("Test Title".to_string());
        schema.notes = Some("Test Note".to_string());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.tasks (user_id, task_title, notes) VALUES ($1, $2, $3) RETURNING task_id"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn date_time_only() {
        let now = Local::now();

        let mut schema = CreateRequest::default();
        schema.start_date = Some(now.date_naive());
        schema.start_time = Some(now.time());
        schema.deadline = Some(now.date_naive());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.tasks (user_id, start_date, start_time, deadline) VALUES ($1, $2, $3, $4) RETURNING task_id"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn id_only() {
        let mut schema = CreateRequest::default();
        schema.area_id = Some(Uuid::new_v4());
        schema.project_id = Some(Uuid::new_v4());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.tasks (user_id, area_id, project_id) VALUES ($1, $2, $3) RETURNING task_id"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = CreateRequest::default();
        schema.title = Some("Test Title".to_string());
        schema.notes = Some("Test Note".to_string());
        schema.start_date = Some(now.date_naive());
        schema.start_time = Some(now.time());
        schema.deadline = Some(now.date_naive());
        schema.area_id = Some(Uuid::new_v4());
        schema.project_id = Some(Uuid::new_v4());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement,
            "INSERT INTO data.tasks (user_id, task_title, notes, start_date, start_time, deadline, area_id, project_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING task_id"
        );
        assert_eq!(params.len(), 8);
    }
}

#[cfg(test)]
mod update_schema {
    use chrono::Local;
    use uuid::Uuid;

    use crate::{models::UpdateMethod, util::ToSQLQueryBuilder};

    use super::UpdateRequest;

    #[test]
    fn is_empty() {
        todo!()
    }

    #[test]
    fn text_only() {
        let mut schema = UpdateRequest::default();
        schema.title = UpdateMethod::Set("Test Title".to_string());
        schema.notes = UpdateMethod::Set("Test Note".to_string());

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, task_title=$2, notes=$3 WHERE user_id = $4 AND task_id = $5 RETURNING task_id"
        );
        assert_eq!(params.len(), 5);
    }

    #[test]
    fn date_time_only() {
        let now = Local::now();

        let mut schema = UpdateRequest::default();
        schema.start_date = UpdateMethod::Set(now.date_naive());
        schema.start_time = UpdateMethod::Set(now.time());
        schema.deadline = UpdateMethod::Set(now.date_naive());

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, start_date=$2, start_time=$3, deadline=$4 WHERE user_id = $5 AND task_id = $6 RETURNING task_id"
        );
        assert_eq!(params.len(), 6);
    }

    #[test]
    fn bool_only() {
        let mut schema = UpdateRequest::default();
        schema.completed = UpdateMethod::Set(true);
        schema.logged = UpdateMethod::Set(true);
        schema.trashed = UpdateMethod::Set(true);

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, completed_on=$2, logged_on=$3, trashed_on=$4 WHERE user_id = $5 AND task_id = $6 RETURNING task_id"
        );
        assert_eq!(params.len(), 6);
    }

    #[test]
    fn id_only() {
        let mut schema = UpdateRequest::default();
        schema.area_id = UpdateMethod::Set(Uuid::new_v4());
        schema.project_id = UpdateMethod::Set(Uuid::new_v4());

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, area_id=$2, project_id=$3 WHERE user_id = $4 AND task_id = $5 RETURNING task_id"
        );
        assert_eq!(params.len(), 5);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = UpdateRequest::default();
        schema.title = UpdateMethod::Set("Test Title".to_string());
        schema.notes = UpdateMethod::Set("Test Note".to_string());
        schema.start_date = UpdateMethod::Set(now.date_naive());
        schema.start_time = UpdateMethod::Set(now.time());
        schema.deadline = UpdateMethod::Set(now.date_naive());
        schema.completed = UpdateMethod::Set(true);
        schema.logged = UpdateMethod::Set(true);
        schema.trashed = UpdateMethod::Set(true);
        schema.area_id = UpdateMethod::Set(Uuid::new_v4());
        schema.project_id = UpdateMethod::Set(Uuid::new_v4());

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, task_title=$2, notes=$3, start_date=$4, start_time=$5, deadline=$6, completed_on=$7, logged_on=$8, trashed_on=$9, area_id=$10, project_id=$11 WHERE user_id = $12 AND task_id = $13 RETURNING task_id"
        );
        assert_eq!(params.len(), 13);
    }
}

#[cfg(test)]
mod query_schema {
    use chrono::Local;
    use uuid::Uuid;

    use crate::{models::Compare, util::ToSQLQueryBuilder};

    use super::{QueryMethod, QueryRequest};

    #[test]
    fn empty() {
        let schema = QueryRequest::default();

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE user_id = $1 LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn text_only() {
        let mut schema = QueryRequest::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.notes = Some(QueryMethod::Match("Test Note".to_string()));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE user_id = $1 AND task_title ILIKE '%' || $2 || '%' AND notes ILIKE '%' || $3 || '%' LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn date_time_eq_only() {
        let now = Local::now();

        let mut schema = QueryRequest::default();
        schema.start_date = Some(QueryMethod::Match(now.date_naive()));
        schema.start_time = Some(QueryMethod::Match(now.time()));
        schema.deadline = Some(QueryMethod::Match(now.date_naive()));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE user_id = $1 AND start_date = $2 AND start_time = $3 AND deadline = $4 LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn date_time_cmp_only() {
        let now = Local::now();

        let mut schema = QueryRequest::default();
        schema.start_date = Some(QueryMethod::Compare(now.date_naive(), Compare::Less));
        schema.start_time = Some(QueryMethod::Compare(now.time(), Compare::LessEq));
        schema.deadline = Some(QueryMethod::Compare(now.date_naive(), Compare::Greater));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE user_id = $1 AND start_date < $2 AND start_time <= $3 AND deadline > $4 LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn bool_t_only() {
        let mut schema = QueryRequest::default();
        schema.completed = Some(true);
        schema.logged = Some(true);
        schema.trashed = Some(true);

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE user_id = $1 AND completed_on NOT NULL AND logged_on NOT NULL AND trashed_on NOT NULL LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn bool_f_only() {
        let mut schema = QueryRequest::default();
        schema.completed = Some(false);
        schema.logged = Some(false);
        schema.trashed = Some(false);

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE user_id = $1 AND completed_on IS NULL AND logged_on IS NULL AND trashed_on IS NULL LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn id_only() {
        let mut schema = QueryRequest::default();
        schema.area_id = Some(Uuid::new_v4());
        schema.project_id = Some(Uuid::new_v4());

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE user_id = $1 AND area_id = $2 AND project_id = $3 LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = QueryRequest::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.notes = Some(QueryMethod::Match("Test Note".to_string()));
        schema.start_date = Some(QueryMethod::Match(now.date_naive()));
        schema.start_time = Some(QueryMethod::Match(now.time()));
        schema.deadline = Some(QueryMethod::Compare(now.date_naive(), Compare::Greater));
        schema.completed = Some(false);
        schema.logged = Some(true);
        schema.trashed = Some(false);
        schema.area_id = Some(Uuid::new_v4());
        schema.project_id = Some(Uuid::new_v4());

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE user_id = $1 AND task_title ILIKE '%' || $2 || '%' AND notes ILIKE '%' || $3 || '%' AND start_date = $4 AND start_time = $5 AND deadline > $6 AND completed_on IS NULL AND logged_on NOT NULL AND trashed_on IS NULL AND area_id = $7 AND project_id = $8 LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 8);
    }
}
