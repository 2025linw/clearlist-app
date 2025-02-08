use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use deadpool_postgres::Object;
use serde::Deserialize;
use tokio_postgres::{types::ToSql, Row};
use uuid::Uuid;

use crate::{
    database::{TaskModel, TaskTagModel},
    request::{
        api::{Create, Delete, Query, Retrieve, Update},
        query::{parameter_string, CmpFlag, ToQuery, TIMESTAMP_NULL},
        UpdateMethod,
    },
    response::Error,
};

use super::query::QueryMethod;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TaskPostRequest {
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

impl TaskPostRequest {
    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Create for TaskPostRequest {
    async fn query(&self, conn: &mut Object) -> Result<Row, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Insert Query
        let row = match transaction
            .query_one(&self.statement(), &self.params())
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Tag Insert Query
        if let Some(tag_ids) = &self.tag_ids {
            let task_id: Uuid = row.get(TaskModel::ID);

            let mut params: Vec<&(dyn ToSql + Sync)> = vec![&task_id];
            for tag_id in tag_ids {
                params.push(tag_id);
            }
            let statement = format!(
                "INSERT INTO {} ({}, {}) VALUES {}",
                TaskTagModel::TABLE,
                TaskTagModel::TASK_ID,
                TaskTagModel::TAG_ID,
                (1..params.len())
                    .map(|n| format!("($1, ${})", 1 + n))
                    .collect::<Vec<String>>()
                    .join(", "),
            );

            // TODO: Do something with this value
            let _rows_affected = match transaction.execute(&statement, &params).await {
                Ok(n) => n,
                Err(e) => return Err(Error::DatabaseError(e)),
            };
        }

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(row)
    }
}

impl ToQuery for TaskPostRequest {
    fn statement(&self) -> String {
        let mut columns: Vec<&str> = vec![TaskModel::USER_ID];

        if self.title.is_some() {
            columns.push(TaskModel::TITLE);
        }
        if self.notes.is_some() {
            columns.push(TaskModel::NOTES);
        }
        if self.start_date.is_some() {
            columns.push(TaskModel::START_DATE);
        }
        if self.start_time.is_some() {
            columns.push(TaskModel::START_TIME);
        }
        if self.deadline.is_some() {
            columns.push(TaskModel::DEADLINE);
        }

        if self.area_id.is_some() {
            columns.push(TaskModel::AREA_ID);
        }
        if self.project_id.is_some() {
            columns.push(TaskModel::PROJECT_ID);
        }

        format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
            TaskModel::TABLE,
            columns.join(", "),
            parameter_string(1..=columns.len()),
            TaskModel::ID,
        )
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        if let Some(s) = &self.title {
            params.push(s);
        }
        if let Some(s) = &self.notes {
            params.push(s);
        }
        if let Some(d) = &self.start_date {
            params.push(d);
        }
        if let Some(t) = &self.start_time {
            params.push(t);
        }
        if let Some(d) = &self.deadline {
            params.push(d);
        }

        if let Some(i) = &self.area_id {
            params.push(i);
        }
        if let Some(i) = &self.project_id {
            params.push(i);
        }

        params
    }
}

#[derive(Debug)]
pub struct TaskGetRequest {
    task_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl TaskGetRequest {
    pub fn task_id(&mut self, id: Uuid) -> &mut Self {
        self.task_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Default for TaskGetRequest {
    fn default() -> Self {
        Self {
            task_id: None,

            user_id: None,
        }
    }
}

impl Retrieve for TaskGetRequest {
    async fn query(&self, conn: &mut Object) -> Result<Option<Row>, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Select Query
        let row_opt = match transaction
            .query_opt(&self.statement(), &self.params())
            .await
        {
            Ok(o) => o,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(row_opt)
    }
}

impl ToQuery for TaskGetRequest {
    fn statement(&self) -> String {
        format!(
            "SELECT * FROM {} WHERE {}=$1 AND {}=$2",
            TaskModel::TABLE,
            TaskModel::ID,
            TaskModel::USER_ID,
        )
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            self.task_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ]
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TaskPutRequest {
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

    #[serde(default)]
    timestamp: DateTime<Local>,
    tag_ids: Option<UpdateMethod<Vec<Uuid>>>,
}

impl TaskPutRequest {
    pub fn task_id(&mut self, id: Uuid) -> &mut Self {
        self.task_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Update for TaskPutRequest {
    async fn query(&self, conn: &mut Object) -> Result<Option<Row>, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Update Query
        let row_opt = match transaction
            .query_opt(&self.statement(), &self.params())
            .await
        {
            Ok(o) => o,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Tag Update Query
        if let (Some(r), Some(tag_update)) = (&row_opt, &self.tag_ids) {
            let task_id: Uuid = r.get(TaskModel::ID);

            // Remove tags
            let params: Vec<&(dyn ToSql + Sync)> = vec![&task_id];
            let statement = format!(
                "DELETE FROM {} WHERE {}=$1",
                TaskTagModel::TABLE,
                TaskModel::ID
            );

            // TODO: do something with rows_affected
            let _rows_affected = match transaction.execute(&statement, &params).await {
                Ok(n) => n,
                Err(e) => return Err(Error::DatabaseError(e)),
            };

            // Add tag
            if let UpdateMethod::Change(tag_ids) = tag_update {
                let mut params: Vec<&(dyn ToSql + Sync)> = vec![&task_id];
                for tag_id in tag_ids {
                    params.push(tag_id);
                }
                let statement = format!(
                    "INSERT INTO {} ({}, {}) VALUES {}",
                    TaskTagModel::TABLE,
                    TaskTagModel::TASK_ID,
                    TaskTagModel::TAG_ID,
                    (1..params.len())
                        .map(|n| format!("($1, ${})", 1 + n))
                        .collect::<Vec<String>>()
                        .join(", "),
                );

                // TODO: do something with rows_affected
                let _rows_affected = match transaction.execute(&statement, &params).await {
                    Ok(n) => n,
                    Err(e) => return Err(Error::DatabaseError(e)),
                };
            }
        }

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(row_opt)
    }
}

impl ToQuery for TaskPutRequest {
    fn statement(&self) -> String {
        let mut columns: Vec<&str> = vec![TaskModel::UPDATED];

        if let Some(_) = &self.title {
            columns.push(TaskModel::TITLE);
        }
        if let Some(_) = &self.notes {
            columns.push(TaskModel::NOTES);
        }
        if let Some(_) = &self.start_date {
            columns.push(TaskModel::START_DATE);
        }
        if let Some(_) = &self.start_time {
            columns.push(TaskModel::START_TIME);
        }
        if let Some(_) = &self.deadline {
            columns.push(TaskModel::DEADLINE);
        }
        if let Some(_) = &self.completed {
            columns.push(TaskModel::COMPLETED);
        }
        if let Some(_) = &self.logged {
            columns.push(TaskModel::LOGGED);
        }
        if let Some(_) = &self.trashed {
            columns.push(TaskModel::TRASHED);
        }

        if let Some(_) = &self.area_id {
            columns.push(TaskModel::AREA_ID);
        }
        if let Some(_) = &self.project_id {
            columns.push(TaskModel::PROJECT_ID);
        }

        format!(
            "UPDATE {} SET ({})=({}) WHERE {}=$1 AND {}=$2 RETURNING *",
            TaskModel::TABLE,
            columns.join(", "),
            (0..columns.len())
                .map(|n| format!("${}", 3 + n))
                .collect::<Vec<String>>()
                .join(", "),
            TaskModel::USER_ID,
            TaskModel::ID,
        )
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![
            self.user_id.as_ref().unwrap(),
            self.task_id.as_ref().unwrap(),
            &self.timestamp,
        ];

        if let Some(s) = &self.title {
            params.push(s);
        }
        if let Some(s) = &self.notes {
            params.push(s);
        }
        if let Some(d) = &self.start_date {
            params.push(d);
        }
        if let Some(t) = &self.start_time {
            params.push(t);
        }
        if let Some(d) = &self.deadline {
            params.push(d);
        }
        if let Some(b) = self.completed {
            if b {
                params.push(&self.timestamp);
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }
        if let Some(b) = self.logged {
            if b {
                params.push(&self.timestamp);
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }
        if let Some(b) = self.trashed {
            if b {
                params.push(&self.timestamp);
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }

        if let Some(i) = &self.area_id {
            params.push(i);
        }
        if let Some(i) = &self.project_id {
            params.push(i);
        }

        params
    }
}

#[derive(Debug)]
pub struct TaskDeleteRequest {
    task_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl TaskDeleteRequest {
    pub fn task_id(&mut self, id: Uuid) -> &mut Self {
        self.task_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Default for TaskDeleteRequest {
    fn default() -> Self {
        Self {
            task_id: None,

            user_id: None,
        }
    }
}

impl Delete for TaskDeleteRequest {
    async fn query(&self, conn: &mut Object) -> Result<bool, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Delete Query
        let success = match transaction.execute(&self.statement(), &self.params()).await {
            Ok(0) => false,
            Ok(1) => true,
            Ok(..) => return Err(Error::QueryError("More than one task deleted".to_string())),
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(success)
    }
}

impl ToQuery for TaskDeleteRequest {
    fn statement(&self) -> String {
        let conditions = vec![
            format!("{}=$1", TaskModel::ID),
            format!("{}=$2", TaskModel::USER_ID),
        ];

        format!(
            "WITH deleted AS (DELETE FROM {} WHERE {}) DELETE FROM {} WHERE {}",
            TaskTagModel::TABLE,
            conditions[0],
            TaskModel::TABLE,
            conditions.join(" AND "),
        )
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            self.task_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ]
    }
}

#[derive(Debug, Deserialize)]
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
    pub fn search_query(&mut self, query: String) -> &mut Self {
        self.search_query = Some(query);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Query for TaskQueryRequest {
    async fn query(&self, conn: &mut Object) -> Result<Vec<Row>, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Query
        let mut rows = match transaction.query(&self.statement(), &self.params()).await {
            Ok(r) => r,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Tags Query
        if let (Some(_), Some(QueryMethod::Match(tag_ids))) = (&self.tag_ids, &self.tag_ids) {
            let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
            for tag_id in tag_ids {
                params.push(tag_id);
            }
            let statement = format!(
				"SELECT {1} FROM {0} WHERE {2} IN ({3}) GROUP BY {1} HAVING COUNT(DISTINCT {2}) >= {4}",
				TaskTagModel::TABLE,
				TaskTagModel::TASK_ID,
				TaskTagModel::TAG_ID,
				parameter_string(1..=params.len()),
				params.len(),
			);

			// Get all tasks that have the queried tags
            let matched_tasks: Vec<Uuid> = match transaction.query(&statement, &params).await {
                Ok(r) => r
                    .iter()
                    .map(|o| o.to_owned().get::<&str, Uuid>(TaskTagModel::TASK_ID))
                    .collect(),
                Err(e) => return Err(Error::DatabaseError(e)),
            };

			// Filter out all tasks that have the queried tags
            rows = rows
                .iter()
                .filter(|r| matched_tasks.contains(&r.get::<&str, Uuid>(TaskModel::ID)))
                .map(|r| r.to_owned())
                .collect();
        }

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(rows)
    }
}

impl ToQuery for TaskQueryRequest {
    fn statement(&self) -> String {
        let mut conditions: Vec<String> = vec![format!("{}=$1", TaskModel::USER_ID)];

        let mut n = conditions.len() + 1;
        if self.search_query.is_some() {
            conditions.push(format!("{} LIKE ${}", TaskModel::TITLE, n));

            n += 1;
        }

        let mut condition;
        if let Some(q) = &self.start_date {
            (condition, n) = q.condition_string(TaskModel::START_DATE, n);

            conditions.push(condition);
        }
        if let Some(q) = &self.deadline {
            (condition, n) = q.condition_string(TaskModel::DEADLINE, n);

            conditions.push(condition);
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
        }
        if let Some(q) = &self.project_id {
            (condition, n) = q.condition_string(TaskModel::PROJECT_ID, n);

            conditions.push(condition);
        }

        format!(
            "SELECT * FROM {} WHERE {}",
            TaskModel::TABLE,
            conditions.join(" AND "),
        )
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        if self.search_query.is_some() {
            params.push(self.search_query.as_ref().unwrap())
        }

        if let Some(q) = &self.start_date {
            if let Some(d) = q.get_param() {
                params.push(d);
            }
        }
        if let Some(q) = &self.deadline {
            if let Some(d) = q.get_param() {
                params.push(d);
            }
        }

        if let Some(q) = &self.area_id {
            if let Some(i) = q.get_param() {
                params.push(i);
            }
        }
        if let Some(q) = &self.project_id {
            if let Some(i) = q.get_param() {
                params.push(i);
            }
        }

        return params;
    }
}

#[cfg(test)]
mod tests {
    // TODO: Task request testing
    // Directly call with premade Requests
    use super::*;

    #[test]
    fn test_add() {
        assert!(true);
    }
}
