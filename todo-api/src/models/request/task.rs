use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use deadpool_postgres::{Object, Transaction};
use serde::Deserialize;
use tokio_postgres::{Row, types::ToSql};
use uuid::Uuid;

use crate::{
    error::Error,
    models::db::{TaskModel, TaskTagModel, parameter_values},
    storage::db::{DBDelete, DBInsert, DBQuery, DBSelectAll, DBSelectOne, DBSubquery, DBUpdate},
};

use super::{QueryMethod, UpdateMethod};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TaskCreateRequest {
    title: Option<String>,
    notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,

    user_id: Option<Uuid>,

    tag_ids: Option<Vec<Uuid>>,
}

impl TaskCreateRequest {
    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TaskCreateRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut columns: Vec<&str> = vec![TaskModel::USER_ID];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        if let Some(s) = &self.title {
            columns.push(TaskModel::TITLE);
            params.push(s)
        }
        if let Some(s) = &self.notes {
            columns.push(TaskModel::NOTES);
            params.push(s);
        }
        if let Some(d) = &self.start_date {
            columns.push(TaskModel::START_DATE);
            params.push(d);
        }
        if let Some(t) = &self.start_time {
            columns.push(TaskModel::START_TIME);
            params.push(t);
        }
        if let Some(d) = &self.deadline {
            columns.push(TaskModel::DEADLINE);
            params.push(d);
        }

        if let Some(i) = &self.area_id {
            columns.push(TaskModel::AREA_ID);
            params.push(i);
        }
        if let Some(i) = &self.project_id {
            columns.push(TaskModel::PROJECT_ID);
            params.push(i);
        }

        let statement = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {} AS id",
            TaskModel::TABLE,
            columns.join(","),
            parameter_values(1..=params.len()).join(","),
            TaskModel::ID,
        );

        (statement, params)
    }
}

impl DBSubquery for TaskCreateRequest {
    fn get_subquery<'a>(&'a self, uuid: &'a Uuid) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![uuid, self.tag_ids.as_ref().unwrap()];

        let statement = format!(
            "INSERT INTO {} ({},{}) \
			SELECT T.id, R.id \
			FROM (SELECT $1::uuid AS id) T \
			CROSS JOIN (SELECT UNNEST($2::uuid[]) AS id) R",
            TaskTagModel::TABLE,
            TaskTagModel::TASK_ID,
            TaskTagModel::TAG_ID,
        );

        (statement, params)
    }
}

impl DBInsert for TaskCreateRequest {
    async fn query(&self, transaction: &Transaction<'_>) -> Result<Row, Error> {
        // Insert Task
        let (statement, params) = self.get_query();

        let row: Row = transaction.query_one(&statement, &params).await?;

        // Insert Task Tags
        if self.tag_ids.is_some() {
            let task_id: Uuid = row.get("id");

            let (statement, params) = self.get_subquery(&task_id);

            let _res = transaction.execute(&statement, &params).await?;
            // TODO: catch res value if it is more or less than 1
        }

        Ok(row)
    }
}

#[derive(Debug, Default)]
pub struct TaskRetrieveRequest {
    task_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl TaskRetrieveRequest {
    pub fn set_task_id(&mut self, id: Uuid) -> &mut Self {
        self.task_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TaskRetrieveRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![
            self.task_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ];

        let statement = format!(
            "SELECT * FROM {} WHERE {}=$1 AND {}=$2",
            TaskModel::TABLE,
            TaskModel::ID,
            TaskModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBSelectOne for TaskRetrieveRequest {
    async fn query(&self, conn: &Object) -> Result<Option<Row>, Error> {
        // Retrieve Task
        let (statement, params) = self.get_query();

        let row_opt: Option<Row> = conn.query_opt(&statement, &params).await?;

        Ok(row_opt)
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TaskUpdateRequest {
    task_id: Option<Uuid>,

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

    user_id: Option<Uuid>,

    tag_ids: Option<UpdateMethod<Vec<Uuid>>>,

    #[serde(default = "Local::now")]
    timestamp: DateTime<Local>,
}

impl TaskUpdateRequest {
    pub fn set_task_id(&mut self, id: Uuid) -> &mut Self {
        self.task_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TaskUpdateRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut updates: Vec<String> = vec![format!("{}=$3", TaskModel::UPDATED)];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![
            self.task_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
            &self.timestamp,
        ];

        let mut n = params.len() + 1;

        let mut update;
        if let Some(u) = &self.title {
            (update, n) = u.update_string(TaskModel::TITLE, n);
            updates.push(update);

            if let Some(s) = u.get_param() {
                params.push(s);
            }
        }
        if let Some(u) = &self.notes {
            (update, n) = u.update_string(TaskModel::NOTES, n);
            updates.push(update);

            if let Some(s) = u.get_param() {
                params.push(s);
            }
        }
        if let Some(u) = &self.start_date {
            (update, n) = u.update_string(TaskModel::START_DATE, n);
            updates.push(update);

            if let Some(d) = u.get_param() {
                params.push(d);
            }
        }
        if let Some(u) = &self.start_time {
            (update, n) = u.update_string(TaskModel::START_TIME, n);
            updates.push(update);

            if let Some(t) = u.get_param() {
                params.push(t);
            }
        }
        if let Some(u) = &self.deadline {
            (update, n) = u.update_string(TaskModel::DEADLINE, n);
            updates.push(update);

            if let Some(d) = u.get_param() {
                params.push(d);
            }
        }

        if let Some(b) = self.completed {
            if b {
                updates.push(format!("{}=$3", TaskModel::COMPLETED));
            } else {
                updates.push(format!("{} IS NULL", TaskModel::COMPLETED));
            }
        }
        if let Some(b) = self.logged {
            if b {
                updates.push(format!("{}=$3", TaskModel::LOGGED));
            } else {
                updates.push(format!("{} IS NULL", TaskModel::LOGGED));
            }
        }
        if let Some(b) = self.trashed {
            if b {
                updates.push(format!("{}=$3", TaskModel::TRASHED));
            } else {
                updates.push(format!("{} IS NULL", TaskModel::TRASHED));
            }
        }

        if let Some(u) = &self.area_id {
            (update, n) = u.update_string(TaskModel::AREA_ID, n);
            updates.push(update);

            if let Some(i) = u.get_param() {
                params.push(i);
            }
        }
        if let Some(u) = &self.project_id {
            (update, _) = u.update_string(TaskModel::PROJECT_ID, n);
            updates.push(update);

            if let Some(i) = u.get_param() {
                params.push(i);
            }
        }

        let statement = format!(
            "UPDATE {} SET {} WHERE {}=$1 AND {}=$2 RETURNING *",
            TaskModel::TABLE,
            updates.join(","),
            TaskModel::ID,
            TaskModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBSubquery for TaskUpdateRequest {
    fn get_subquery<'a>(&'a self, uuid: &'a Uuid) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let tag_update: &UpdateMethod<Vec<Uuid>> = self.tag_ids.as_ref().unwrap();

        let mut params: Vec<&(dyn ToSql + Sync)> = vec![uuid];

        let mut statement = format!(
            "DELETE FROM {} WHERE {}=$1 RETURNING {} AS id",
            TaskTagModel::TABLE,
            TaskTagModel::TASK_ID,
            TaskTagModel::TASK_ID,
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
                TaskTagModel::TABLE,
                TaskTagModel::TASK_ID,
                TaskTagModel::TAG_ID,
            );
        }

        (statement, params)
    }
}

impl DBUpdate for TaskUpdateRequest {
    async fn query(&self, transaction: &Transaction<'_>) -> Result<Option<Row>, Error> {
        // Update Task
        let (statement, params) = self.get_query();

        let row_opt: Option<Row> = transaction.query_opt(&statement, &params).await?;

        // Update Task Tags
        if let (Some(row), Some(_)) = (&row_opt, &self.tag_ids) {
            let task_id: Uuid = row.get(TaskModel::ID);

            let (statement, params) = self.get_subquery(&task_id);

            let _res = transaction.execute(&statement, &params).await?;
            // TODO: do something with res (if less than or greater than 1, etc)
        }

        Ok(row_opt)
    }
}

#[derive(Debug, Default)]
pub struct TaskDeleteRequest {
    task_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl TaskDeleteRequest {
    pub fn set_task_id(&mut self, id: Uuid) -> &mut Self {
        self.task_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TaskDeleteRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![
            self.task_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ];

        let statement = format!(
            "DELETE FROM {} WHERE {}=$1 AND {}=$2",
            TaskModel::TABLE,
            TaskModel::ID,
            TaskModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBDelete for TaskDeleteRequest {
    async fn query(&self, transaction: &Transaction<'_>) -> Result<bool, Error> {
        // Delete Task
        let (statement, params) = self.get_query();

        let res = transaction.execute(&statement, &params).await?;

        match res {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::Internal), // TODO: better error?
        }
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TaskQueryRequest {
    search_query: Option<String>,

    start_date: Option<QueryMethod<NaiveDate>>,
    deadline: Option<QueryMethod<NaiveDate>>,

    completed: Option<QueryMethod<bool>>,
    logged: Option<QueryMethod<bool>>,
    trashed: Option<QueryMethod<bool>>,

    area_id: Option<QueryMethod<Uuid>>,
    project_id: Option<QueryMethod<Uuid>>,

    user_id: Option<Uuid>,

    tag_ids: Option<QueryMethod<Vec<Uuid>>>,
}

impl TaskQueryRequest {
    pub fn set_search_query(&mut self, query: String) -> &mut Self {
        self.search_query = Some(format!("%{}%", query));

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TaskQueryRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut conditions: Vec<String> = vec![format!("{}=$1", TaskModel::USER_ID)];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        let mut n = conditions.len() + 1;

        if let Some(s) = &self.search_query {
            conditions.push(format!("{} LIKE ${}", TaskModel::TITLE, n));
            params.push(s);

            n += 1;
        }

        let mut condition;
        if let Some(q) = &self.start_date {
            (condition, n) = q.condition_string(TaskModel::START_DATE, n);
            conditions.push(condition);

            if let Some(d) = q.get_param() {
                params.push(d);
            }
        }
        if let Some(q) = &self.deadline {
            (condition, n) = q.condition_string(TaskModel::DEADLINE, n);
            conditions.push(condition);

            if let Some(d) = q.get_param() {
                params.push(d);
            }
        }

        if let Some(q) = &self.completed {
            (condition, n) = q.condition_string(TaskModel::COMPLETED, n);
            conditions.push(condition);
        }
        if let Some(q) = &self.logged {
            (condition, n) = q.condition_string(TaskModel::LOGGED, n);
            conditions.push(condition);
        }
        if let Some(q) = &self.trashed {
            (condition, n) = q.condition_string(TaskModel::TRASHED, n);
            conditions.push(condition);
        }

        if let Some(q) = &self.area_id {
            (condition, n) = q.condition_string(TaskModel::AREA_ID, n);
            conditions.push(condition);

            if let Some(i) = q.get_param() {
                params.push(i);
            }
        }
        if let Some(q) = &self.project_id {
            (condition, _) = q.condition_string(TaskModel::PROJECT_ID, n);
            conditions.push(condition);

            if let Some(i) = q.get_param() {
                params.push(i);
            }
        }

        let statement = format!(
            "SELECT * FROM {} WHERE {}",
            TaskModel::TABLE,
            conditions.join(" AND "),
        );

        (statement, params)
    }
}

impl DBSubquery for TaskQueryRequest {
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
				GROUP BY task_id \
				HAVING COUNT(DISTINCT tag_id) >= {}",
                TaskTagModel::TASK_ID,
                TaskTagModel::TABLE,
                TaskTagModel::TAG_ID,
                tag_ids.len(),
            );
        } else {
            params = vec![];

            statement = Default::default();
        }

        (statement, params)
    }
}

impl DBSelectAll for TaskQueryRequest {
    async fn query(&self, conn: &Object) -> Result<Vec<Row>, Error> {
        // Get Tasks
        let (statement, params) = self.get_query();

        let mut rows = conn.query(&statement, &params).await?;

        // Get Task Tags and Filter
        if let Some(QueryMethod::Match(_)) = self.tag_ids {
            let null_uuid = Uuid::nil();

            let (statement, params) = self.get_subquery(&null_uuid);

            let tag_rows: Vec<Uuid> = conn
                .query(&statement, &params)
                .await?
                .iter()
                .map(|r| r.get(TaskTagModel::TASK_ID))
                .collect();

            // Filter Tasks with Task Tags
            rows = rows
                .iter()
                .filter(|r| {
                    let task_id: Uuid = r.get(TaskModel::ID);

                    tag_rows.contains(&task_id)
                })
                .map(|r| r.to_owned())
                .collect();
        }

        Ok(rows)
    }
}

#[cfg(test)]
mod create_query {
    use deadpool_postgres::Pool;
    use dotenvy::dotenv;
	use std::{env, sync::atomic::AtomicBool};
	use futures::executor::block_on;
	use std::sync::OnceLock;

	// TODO: make tests
	use crate::get_database_pool;

    use super::*;

    fn request_setup() -> TaskCreateRequest {
        let mut request = TaskCreateRequest::default();
        request.user_id = Some(Uuid::nil());

        request
    }

	#[test]
    fn blank() {
        let request = request_setup();

        let (query, params) = request.get_query();

        assert_eq!(
			query.as_str(),
			"INSERT INTO data.tasks (user_id) VALUES ($1) RETURNING task_id AS id",
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn title() {
        let mut request = request_setup();
        request.title = Some("Test Task".to_string());

        let (query, params) = request.get_query();

        assert_eq!(
            query.as_str(),
			"INSERT INTO data.tasks (user_id,task_title) VALUES ($1,$2) RETURNING task_id AS id",
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn notes() {
        let mut request = request_setup();
        request.notes = Some("Test Notes".to_string());

        let (query, params) = request.get_query();

        assert_eq!(
            query.as_str(),
			"INSERT INTO data.tasks (user_id,task_notes) VALUES ($1,$2) RETURNING task_id AS id",
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn start_date() {
        let date = Local::now().date_naive();

        let mut request = request_setup();
        request.start_date = Some(date);

        let (query, params) = request.get_query();

        assert_eq!(
            query.as_str(),
			"INSERT INTO data.tasks (user_id,start_date) VALUES ($1,$2) RETURNING task_id AS id",
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn start_time() {
        let now = Local::now();
        let date = now.date_naive();
        let time = now.time();

        let mut request = request_setup();
        request.start_date = Some(date);
        request.start_time = Some(time);

        let (query, params) = request.get_query();

        assert_eq!(
            query.as_str(),
			"INSERT INTO data.tasks (user_id,start_date,start_time) VALUES ($1,$2,$3) RETURNING task_id AS id",
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn deadline() {
        let date = Local::now().date_naive();

        let mut request = request_setup();
        request.deadline = Some(date);

        let (query, params) = request.get_query();

        assert_eq!(
            query.as_str(),
			"INSERT INTO data.tasks (user_id,deadline) VALUES ($1,$2) RETURNING task_id AS id",
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn area_id() {
        let mut request = request_setup();
        request.area_id = Some(Uuid::nil());

        let (query, params) = request.get_query();

        assert_eq!(
            query.as_str(),
			"INSERT INTO data.tasks (user_id,area_id) VALUES ($1,$2) RETURNING task_id AS id",
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn project_id() {
        let mut request = request_setup();
        request.project_id = Some(Uuid::nil());

        let (query, params) = request.get_query();

        assert_eq!(
            query.as_str(),
            "INSERT INTO data.tasks (user_id,project_id) VALUES ($1,$2) RETURNING task_id AS id",
        );
        assert_eq!(params.len(), 2);
    }
}

#[cfg(test)]
mod retrieve_query {
    use super::*;

    fn request_setup() -> TaskRetrieveRequest {
        let mut request = TaskRetrieveRequest::default();
        request.user_id = Some(Uuid::nil());
        request.task_id = Some(Uuid::nil());

        request
    }

    #[test]
    fn request() {
        let request = request_setup();

        let (query, params) = request.get_query();

        assert_eq!(
            query.as_str(),
            "SELECT * FROM data.tasks WHERE task_id=$1 AND user_id=$2",
        );
        assert_eq!(params.len(), 2);
    }
}

#[cfg(test)]
mod update_query {
	use super::*;

	fn request_setup() -> TaskUpdateRequest {
		let mut request = TaskUpdateRequest::default();
		request.user_id = Some(Uuid::nil());

		request
	}

	#[test]
	fn title() {
		let mut request = request_setup();


	}
}
