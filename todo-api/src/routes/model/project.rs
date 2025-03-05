use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use deadpool_postgres::{Object, Transaction};
use serde::Deserialize;
use tokio_postgres::{Row, types::ToSql};
use uuid::Uuid;

use crate::{
    error::Error,
    storage::{
        db::{DBDelete, DBInsert, DBQuery, DBSelectAll, DBSelectOne, DBSubquery, DBUpdate},
        model::{ProjectModel, ProjectTagModel},
    },
    util::parameter_values,
};

use super::{QueryMethod, UpdateMethod};

const TIMESTAMP_NULL: &Option<DateTime<Local>> = &None;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProjectCreateRequest {
    title: Option<String>,
    notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,

    user_id: Option<Uuid>,

    tag_ids: Option<Vec<Uuid>>,
}

impl ProjectCreateRequest {
    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for ProjectCreateRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut columns: Vec<&str> = vec![ProjectModel::USER_ID];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        if let Some(s) = &self.title {
            columns.push(ProjectModel::TITLE);
            params.push(s);
        }
        if let Some(s) = &self.notes {
            columns.push(ProjectModel::NOTES);
            params.push(s);
        }
        if let Some(d) = &self.start_date {
            columns.push(ProjectModel::START_DATE);
            params.push(d);
        }
        if let Some(t) = &self.start_time {
            columns.push(ProjectModel::START_TIME);
            params.push(t);
        }
        if let Some(d) = &self.deadline {
            columns.push(ProjectModel::DEADLINE);
            params.push(d);
        }

        if let Some(i) = &self.area_id {
            columns.push(ProjectModel::AREA_ID);
            params.push(i);
        }

        let statement = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {} AS id",
            ProjectModel::TABLE,
            columns.join(","),
            parameter_values(1..=params.len()).join(","),
            ProjectModel::ID,
        );

        (statement, params)
    }
}

impl DBSubquery for ProjectCreateRequest {
    fn get_subquery<'a>(&'a self, uuid: &'a Uuid) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![uuid, self.tag_ids.as_ref().unwrap()];

        let statement = format!(
            "INSERT INTO {} ({},{}) \
			SELECT T.id, R.id \
			FROM (SELECT $1::uuid AS id) T \
			CROSS JOIN (SELECT UNNEST($2::uuid[]) AS id) R",
            ProjectTagModel::TABLE,
            ProjectTagModel::PROJECT_ID,
            ProjectTagModel::TAG_ID,
        );

        (statement, params)
    }
}

impl DBInsert for ProjectCreateRequest {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<Row, Error> {
        // Insert Project
        let (statement, params) = self.get_query();

        let row: Row = transaction.query_one(&statement, &params).await?;

        // Insert Project Tags
        if let Some(_) = self.tag_ids {
            let project_id: Uuid = row.get("id");

            let (statement, params) = self.get_subquery(&project_id);

            let _res = transaction.execute(&statement, &params).await?;
            // TODO: catch res value if it is more or less than 1
        }

        Ok(row)
    }
}

#[derive(Debug, Default)]
pub struct ProjectRetrieveRequest {
    project_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl ProjectRetrieveRequest {
    pub fn set_project_id(&mut self, id: Uuid) -> &mut Self {
        self.project_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for ProjectRetrieveRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![
            self.project_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ];

        let statement = format!(
            "SELECT * FROM {} WHERE {}=$1 AND {}=$2",
            ProjectModel::TABLE,
            ProjectModel::ID,
            ProjectModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBSelectOne for ProjectRetrieveRequest {
    async fn query(&self, conn: &Object) -> Result<Option<Row>, Error> {
        // Retrieve Project
        let (statement, params) = self.get_query();

        let row_opt: Option<Row> = conn.query_opt(&statement, &params).await?;

        Ok(row_opt)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProjectUpdateRequest {
    project_id: Option<Uuid>,

    title: Option<UpdateMethod<String>>,
    notes: Option<UpdateMethod<String>>,
    start_date: Option<UpdateMethod<NaiveDate>>,
    start_time: Option<UpdateMethod<NaiveTime>>,
    deadline: Option<UpdateMethod<NaiveDate>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    area_id: Option<UpdateMethod<Uuid>>,

    user_id: Option<Uuid>,

    tag_ids: Option<UpdateMethod<Vec<Uuid>>>,

    #[serde(default = "Local::now")]
    timestamp: DateTime<Local>,
}

impl ProjectUpdateRequest {
    pub fn set_project_id(&mut self, id: Uuid) -> &mut Self {
        self.project_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for ProjectUpdateRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut columns: Vec<&str> = vec![ProjectModel::UPDATED];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![
            self.project_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
            &self.timestamp,
        ];

        if let Some(u) = &self.title {
            columns.push(ProjectModel::TITLE);
            if let Some(s) = u.get_param() {
                params.push(s);
            }
        }
        if let Some(u) = &self.notes {
            columns.push(ProjectModel::NOTES);
            if let Some(s) = u.get_param() {
                params.push(s);
            }
        }
        if let Some(u) = &self.start_date {
            columns.push(ProjectModel::START_DATE);
            if let Some(d) = u.get_param() {
                params.push(d);
            }
        }
        if let Some(u) = &self.start_time {
            columns.push(ProjectModel::START_TIME);
            if let Some(t) = u.get_param() {
                params.push(t);
            }
        }
        if let Some(u) = &self.deadline {
            columns.push(ProjectModel::DEADLINE);
            if let Some(d) = u.get_param() {
                params.push(d);
            }
        }

        if let Some(b) = self.completed {
            columns.push(ProjectModel::COMPLETED);
            if b {
                params.push(&self.timestamp);
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }
        if let Some(b) = self.logged {
            columns.push(ProjectModel::LOGGED);
            if b {
                params.push(&self.timestamp);
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }
        if let Some(b) = self.trashed {
            columns.push(ProjectModel::TRASHED);
            if b {
                params.push(&self.timestamp);
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }

        if let Some(u) = &self.area_id {
            columns.push(ProjectModel::AREA_ID);
            if let Some(i) = u.get_param() {
                params.push(i);
            }
        }

        let statement = format!(
            "UPDATE {} SET ({})=({}) WHERE {}=$1 AND {}=$2 RETURNING *",
            ProjectModel::TABLE,
            columns.join(","),
            (0..columns.len()) // TODO: change this to use params.len
                .map(|n| format!("${}", 3 + n))
                .collect::<Vec<String>>()
                .join(","),
            ProjectModel::ID,
            ProjectModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBSubquery for ProjectUpdateRequest {
    fn get_subquery<'a>(&'a self, uuid: &'a Uuid) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let tag_update: &UpdateMethod<Vec<Uuid>> = self.tag_ids.as_ref().unwrap();

        let mut params: Vec<&(dyn ToSql + Sync)> = vec![uuid];

        let mut statement = format!(
            "DELETE FROM {} WHERE {}=$1 RETURNING {} as id",
            ProjectTagModel::TABLE,
            ProjectTagModel::PROJECT_ID,
            ProjectTagModel::PROJECT_ID,
        );

        if let UpdateMethod::Change(tag_ids) = tag_update {
            params.push(tag_ids);

            statement = format!(
                "WITH deleted AS ({}) \
				INSERT INTO {} ({},{}) \
				SELECT T.id, R.id \
				FROM (SELECT DISTINCT id FROM deleted) T \
				CROSS JOIN (SELECT UNNEST($2::uuid[]) AS id) R",
                statement,
                ProjectTagModel::TABLE,
                ProjectTagModel::PROJECT_ID,
                ProjectTagModel::TAG_ID,
            );
        }

        (statement, params)
    }
}

impl DBUpdate for ProjectUpdateRequest {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<Option<Row>, Error> {
        // Update Project
        let (statement, params) = self.get_query();

        let row_opt: Option<Row> = transaction.query_opt(&statement, &params).await?;

        // Update Project Tags
        if let (Some(row), Some(_)) = (&row_opt, &self.tag_ids) {
            let project_id: Uuid = row.get(ProjectModel::ID);

            let (statement, params) = self.get_subquery(&project_id);

            let _res = transaction.execute(&statement, &params).await?;
            // TODO: deal with res
        }

        Ok(row_opt)
    }
}

#[derive(Debug, Default)]
pub struct ProjectDeleteRequest {
    project_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl ProjectDeleteRequest {
    pub fn set_project_id(&mut self, id: Uuid) -> &mut Self {
        self.project_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for ProjectDeleteRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![
            self.project_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ];

        let statement = format!(
            "DELETE FROM {} WHERE {}=$1 AND {}=$2",
            ProjectModel::TABLE,
            ProjectModel::ID,
            ProjectModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBDelete for ProjectDeleteRequest {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<bool, Error> {
        // Delete Project
        let (statement, params) = self.get_query();

        let res = transaction.execute(&statement, &params).await?;

        match res {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::Internal), // TODO: find better error
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProjectQueryRequest {
    search_query: Option<String>,

    start_date: Option<QueryMethod<NaiveDate>>,
    deadline: Option<QueryMethod<NaiveDate>>,

    completed: Option<QueryMethod<bool>>,
    logged: Option<QueryMethod<bool>>,
    trashed: Option<QueryMethod<bool>>,

    area_id: Option<QueryMethod<Uuid>>,

    user_id: Option<Uuid>,

    tag_ids: Option<QueryMethod<Vec<Uuid>>>,
}

impl ProjectQueryRequest {
    pub fn set_search_query(&mut self, query: String) -> &mut Self {
        self.search_query = Some(format!("%{}%", query));

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for ProjectQueryRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut conditions: Vec<String> = vec![format!("{}=$1", ProjectModel::USER_ID)];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        let mut n = conditions.len() + 1;

        if let Some(s) = &self.search_query {
            conditions.push(format!("{} LIKE ${}", ProjectModel::TITLE, n));
            params.push(s);

            n += 1;
        }

        let mut condition;
        if let Some(q) = &self.start_date {
            (condition, n) = q.condition_string(ProjectModel::START_DATE, n);
            conditions.push(condition);

            if let Some(d) = q.get_param() {
                params.push(d);
            }
        }
        if let Some(q) = &self.deadline {
            (condition, n) = q.condition_string(ProjectModel::DEADLINE, n);
            conditions.push(condition);

            if let Some(d) = q.get_param() {
                params.push(d);
            }
        }

        if let Some(q) = &self.completed {
            (condition, n) = q.condition_string(ProjectModel::COMPLETED, n);
            conditions.push(condition);
        }
        if let Some(q) = &self.logged {
            (condition, n) = q.condition_string(ProjectModel::LOGGED, n);

            conditions.push(condition);
        }
        if let Some(q) = &self.trashed {
            (condition, n) = q.condition_string(ProjectModel::TRASHED, n);

            conditions.push(condition);
        }

        if let Some(q) = &self.area_id {
            (condition, _) = q.condition_string(ProjectModel::AREA_ID, n);
            conditions.push(condition);

            if let Some(i) = q.get_param() {
                params.push(i);
            }
        }

        let statement = format!(
            "SELECT * FROM {} WHERE {}",
            ProjectModel::TABLE,
            conditions.join(" AND "),
        );

        (statement, params)
    }
}

impl DBSubquery for ProjectQueryRequest {
    fn get_subquery<'a>(&'a self, _: &'a Uuid) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let tag_query = self.tag_ids.as_ref().unwrap();

        let statement;
        let params: Vec<&(dyn ToSql + Sync)>;

        if let QueryMethod::Match(tag_ids) = tag_query {
            params = vec![tag_ids];

            statement = format!(
                "SELECT DISTINCT {} \
				FROM {} \
				WHERE {} IN (SELECT UNNEST($1::uuid[])) \
				GROUP BY project_id	\
				HAVING COUNT(DISTINCT tag_id) >= {}",
                ProjectTagModel::PROJECT_ID,
                ProjectTagModel::TABLE,
                ProjectTagModel::TAG_ID,
                tag_ids.len(),
            );
        } else {
            params = vec![];

            statement = Default::default();
        }

        (statement, params)
    }
}

impl DBSelectAll for ProjectQueryRequest {
    async fn query(&self, conn: &Object) -> Result<Vec<Row>, Error> {
        // Get Projects
        let (statement, params) = self.get_query();

        let mut rows = conn.query(&statement, &params).await?;

        // Get Project Tags and Filter
        if let Some(QueryMethod::Match(_)) = self.tag_ids {
            let null_uuid = Uuid::nil();

            let (statement, params) = self.get_subquery(&null_uuid);

            let tag_rows: Vec<Uuid> = conn
                .query(&statement, &params)
                .await?
                .iter()
                .map(|r| r.get(ProjectTagModel::PROJECT_ID))
                .collect();

            rows = rows
                .iter()
                .filter(|r| {
                    let project_id: Uuid = r.get(ProjectModel::ID);

                    tag_rows.contains(&project_id)
                })
                .map(|r| r.to_owned())
                .collect();
        }

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    // use super::*;

    #[test]
    fn placeholder() {
        unimplemented!()
    }
}
