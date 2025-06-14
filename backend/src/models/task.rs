pub mod tag;

use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::util::{NULL, PostgresCmp, SQLQueryBuilder, ToPostgresCmp, ToSQLQueryBuilder};

use super::{QueryMethod, ToResponse, UpdateMethod};

/// Task Database Model
#[derive(Debug, Deserialize)]
pub struct DatabaseModel {
    #[serde(alias = "task_id")]
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

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl DatabaseModel {
    pub fn id(&self) -> Uuid {
        self.id
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
            tag_ids: Vec::new(),
            user_id: self.user_id,
            created_on: self.created_on,
            updated_on: self.updated_on,
        }
    }
}

/// Task Response Model
#[derive(Debug, Deserialize, Serialize)]
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

    tag_ids: Vec<Uuid>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl ResponseModel {
    pub fn add_tag_ids(mut self, tag_ids: Vec<Uuid>) -> Self {
        self.tag_ids = tag_ids;

        self
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct CreateSchema {
    title: Option<String>,
    notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,

    tag_ids: Vec<Uuid>,
}

impl CreateSchema {
    fn tag_ids(&self) -> &[Uuid] {
        &self.tag_ids
    }
}

impl ToSQLQueryBuilder for CreateSchema {
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

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct UpdateSchema {
    title: Option<UpdateMethod<String>>,
    notes: Option<UpdateMethod<String>>,
    start_date: Option<UpdateMethod<NaiveDate>>,
    start_time: Option<UpdateMethod<NaiveTime>>,
    deadline: Option<UpdateMethod<NaiveDate>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    area_id: Option<UpdateMethod<Uuid>>,
    project_id: Option<UpdateMethod<Uuid>>,

    tag_ids: Vec<Uuid>,

    #[serde(default)]
    timestamp: DateTime<Local>,
}

impl UpdateSchema {
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.notes.is_none()
            && self.start_date.is_none()
            && self.start_time.is_none()
            && self.deadline.is_none()
            && self.completed.is_none()
            && self.logged.is_none()
            && self.trashed.is_none()
            && self.area_id.is_none()
            && self.project_id.is_none()
    }
}

impl ToSQLQueryBuilder for UpdateSchema {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_column(DatabaseModel::UPDATED, &self.timestamp);
        builder.set_return(&[DatabaseModel::ID]);

        if let Some(ref u) = self.title {
            builder.add_column(DatabaseModel::TITLE, u);
        }
        if let Some(ref u) = self.notes {
            builder.add_column(DatabaseModel::NOTES, u);
        }
        if let Some(ref u) = self.start_date {
            builder.add_column(DatabaseModel::START_DATE, u);
        }
        if let Some(ref u) = self.start_time {
            builder.add_column(DatabaseModel::START_TIME, u);
        }
        if let Some(ref u) = self.deadline {
            builder.add_column(DatabaseModel::DEADLINE, u);
        }

        if let Some(b) = self.completed {
            if b {
                builder.add_column(DatabaseModel::COMPLETED, &self.timestamp);
            } else {
                builder.add_column(DatabaseModel::COMPLETED, &None::<DateTime<Local>>);
            }
        }
        if let Some(b) = self.logged {
            if b {
                builder.add_column(DatabaseModel::LOGGED, &self.timestamp);
            } else {
                builder.add_column(DatabaseModel::LOGGED, &None::<DateTime<Local>>);
            }
        }
        if let Some(b) = self.trashed {
            if b {
                builder.add_column(DatabaseModel::TRASHED, &self.timestamp);
            } else {
                builder.add_column(DatabaseModel::TRASHED, &None::<DateTime<Local>>);
            }
        }

        if let Some(ref u) = self.area_id {
            builder.add_column(DatabaseModel::AREA_ID, u);
        }
        if let Some(ref u) = self.project_id {
            builder.add_column(DatabaseModel::PROJECT_ID, u);
        }

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct QuerySchema {
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

impl ToSQLQueryBuilder for QuerySchema {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);

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
mod create_schema_test {
    use chrono::Local;
    use uuid::Uuid;

    use crate::util::ToSQLQueryBuilder;

    use super::CreateSchema;

    #[test]
    fn text_only() {
        let mut schema = CreateSchema::default();
        schema.title = Some("Test Title".to_string());
        schema.notes = Some("Test Note".to_string());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.tasks (task_title, notes) VALUES ($1, $2) RETURNING task_id"
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn date_time_only() {
        let now = Local::now();

        let mut schema = CreateSchema::default();
        schema.start_date = Some(now.date_naive());
        schema.start_time = Some(now.time());
        schema.deadline = Some(now.date_naive());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.tasks (start_date, start_time, deadline) VALUES ($1, $2, $3) RETURNING task_id"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn id_only() {
        let mut schema = CreateSchema::default();
        schema.area_id = Some(Uuid::new_v4());
        schema.project_id = Some(Uuid::new_v4());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.tasks (area_id, project_id) VALUES ($1, $2) RETURNING task_id"
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = CreateSchema::default();
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
            "INSERT INTO data.tasks (task_title, notes, start_date, start_time, deadline, area_id, project_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING task_id"
        );
        assert_eq!(params.len(), 7);
    }

    // TEST: make production example
}

#[cfg(test)]
mod update_schema_test {
    use chrono::Local;
    use uuid::Uuid;

    use crate::{models::UpdateMethod, util::ToSQLQueryBuilder};

    use super::UpdateSchema;

    #[test]
    fn text_only() {
        let mut schema = UpdateSchema::default();
        schema.title = Some(UpdateMethod::Set("Test Title".to_string()));
        schema.notes = Some(UpdateMethod::Set("Test Note".to_string()));

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, task_title=$2, notes=$3 RETURNING task_id"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn date_time_only() {
        let now = Local::now();

        let mut schema = UpdateSchema::default();
        schema.start_date = Some(UpdateMethod::Set(now.date_naive()));
        schema.start_time = Some(UpdateMethod::Set(now.time()));
        schema.deadline = Some(UpdateMethod::Set(now.date_naive()));

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, start_date=$2, start_time=$3, deadline=$4 RETURNING task_id"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn bool_only() {
        let mut schema = UpdateSchema::default();
        schema.completed = Some(true);
        schema.logged = Some(true);
        schema.trashed = Some(true);

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, completed_on=$2, logged_on=$3, trashed_on=$4 RETURNING task_id"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn id_only() {
        let mut schema = UpdateSchema::default();
        schema.area_id = Some(UpdateMethod::Set(Uuid::new_v4()));
        schema.project_id = Some(UpdateMethod::Set(Uuid::new_v4()));

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, area_id=$2, project_id=$3 RETURNING task_id"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = UpdateSchema::default();
        schema.title = Some(UpdateMethod::Set("Test Title".to_string()));
        schema.notes = Some(UpdateMethod::Set("Test Note".to_string()));
        schema.start_date = Some(UpdateMethod::Set(now.date_naive()));
        schema.start_time = Some(UpdateMethod::Set(now.time()));
        schema.deadline = Some(UpdateMethod::Set(now.date_naive()));
        schema.completed = Some(true);
        schema.logged = Some(true);
        schema.trashed = Some(true);
        schema.area_id = Some(UpdateMethod::Set(Uuid::new_v4()));
        schema.project_id = Some(UpdateMethod::Set(Uuid::new_v4()));

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tasks SET updated_on=$1, task_title=$2, notes=$3, start_date=$4, start_time=$5, deadline=$6, completed_on=$7, logged_on=$8, trashed_on=$9, area_id=$10, project_id=$11 RETURNING task_id"
        );
        assert_eq!(params.len(), 11);
    }

    // TEST: make production example
}

#[cfg(test)]
mod query_schema_test {
    use chrono::Local;
    use uuid::Uuid;

    use crate::{models::Compare, util::ToSQLQueryBuilder};

    use super::{QueryMethod, QuerySchema};

    #[test]
    fn empty() {
        let schema = QuerySchema::default();

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(statement.as_str(), "SELECT * FROM data.tasks");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn text_only() {
        let mut schema = QuerySchema::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.notes = Some(QueryMethod::Match("Test Note".to_string()));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE task_title ILIKE '%' || $1 || '%' AND notes ILIKE '%' || $2 || '%'"
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn date_time_eq_only() {
        let now = Local::now();

        let mut schema = QuerySchema::default();
        schema.start_date = Some(QueryMethod::Match(now.date_naive()));
        schema.start_time = Some(QueryMethod::Match(now.time()));
        schema.deadline = Some(QueryMethod::Match(now.date_naive()));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE start_date = $1 AND start_time = $2 AND deadline = $3"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn date_time_cmp_only() {
        let now = Local::now();

        let mut schema = QuerySchema::default();
        schema.start_date = Some(QueryMethod::Compare(now.date_naive(), Compare::Less));
        schema.start_time = Some(QueryMethod::Compare(now.time(), Compare::LessEq));
        schema.deadline = Some(QueryMethod::Compare(now.date_naive(), Compare::Greater));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE start_date < $1 AND start_time <= $2 AND deadline > $3"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn bool_t_only() {
        let mut schema = QuerySchema::default();
        schema.completed = Some(true);
        schema.logged = Some(true);
        schema.trashed = Some(true);

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE completed_on NOT NULL AND logged_on NOT NULL AND trashed_on NOT NULL"
        );
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn bool_f_only() {
        let mut schema = QuerySchema::default();
        schema.completed = Some(false);
        schema.logged = Some(false);
        schema.trashed = Some(false);

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE completed_on IS NULL AND logged_on IS NULL AND trashed_on IS NULL"
        );
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn id_only() {
        let mut schema = QuerySchema::default();
        schema.area_id = Some(Uuid::new_v4());
        schema.project_id = Some(Uuid::new_v4());

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tasks WHERE area_id = $1 AND project_id = $2"
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = QuerySchema::default();
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
            "SELECT * FROM data.tasks WHERE task_title ILIKE '%' || $1 || '%' AND notes ILIKE '%' || $2 || '%' AND start_date = $3 AND start_time = $4 AND deadline > $5 AND completed_on IS NULL AND logged_on NOT NULL AND trashed_on IS NULL AND area_id = $6 AND project_id = $7"
        );
        assert_eq!(params.len(), 7);
    }

    // TEST: test with tags

    // TEST: make production example
}
