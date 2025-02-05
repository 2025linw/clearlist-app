use deadpool_postgres::Object;
use serde::Deserialize;
use tokio_postgres::{types::ToSql, Row};

use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::{
    database::{TaskModel, TaskTagModel},
    request::{
        api::{Create, Delete, Info, Query, Retrieve, Update},
        query::CmpFlag,
        DateFilter, UpdateMethod, TIMESTAMP_NULL,
    },
    response::Error,
};

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
    pub fn user_id(&mut self, id: Uuid) {
        self.user_id = Some(id);
    }
}

impl Create for TaskPostRequest {
    fn columns_vec(&self) -> Vec<String> {
        let mut columns: Vec<String> = Vec::new();

        let mut statement = format!(
            "INSERT INTO todo_data.tasks \
			("
        );

        if self.title.is_some() {
            columns.push(TaskModel::TITLE.to_string());
        }
        if self.notes.is_some() {
            columns.push(TaskModel::NOTES.to_string());
        }

        if self.start_date.is_some() {
            columns.push(TaskModel::START_DATE.to_string());
        }
        if self.start_time.is_some() {
            columns.push(TaskModel::START_TIME.to_string());
        }
        if self.deadline.is_some() {
            columns.push(TaskModel::DEADLINE.to_string());
        }

        if self.area_id.is_some() {
            columns.push(TaskModel::AREA_ID.to_string());
        }
        if self.project_id.is_some() {
            columns.push(TaskModel::PROJECT_ID.to_string());
        }

        if self.user_id.is_some() {
            columns.push(TaskModel::USER_ID.to_string());
        }

        columns
    }

    fn params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

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

    async fn insert_query(&self, conn: &mut Object, info: Option<Info>) -> Result<Row, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Setup
        let info = info.unwrap();

        // Task Insert Query
        let mut params = self.params();
        params.splice(0..0, [info.user_id() as &(dyn ToSql + Sync)]);

        let statement = format!(
            "INSERT INTO todo_data.tasks \
			({}, {}) \
			VALUES \
			($1, {}) \
			RETURNING task_id",
            TaskModel::USER_ID,
            self.columns_vec().join(", "),
            (1..params.len())
                .map(|n| format!("${}", 1 + n))
                .collect::<Vec<String>>()
                .join(", "),
        );

        let row = match transaction.query_one(&statement, &params).await {
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
                "INSERT INTO todo_data.task_tags \
				({}, {}) \
				VALUES \
				{}",
                TaskTagModel::TASK_ID,
                TaskTagModel::TAG_ID,
                (1..params.len())
                    .map(|n| format!("($1, ${})", 1 + n))
                    .collect::<Vec<String>>()
                    .join(", "),
            );

            let rows_affected = match transaction.execute(&statement, &params).await {
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

#[derive(Debug)]
pub struct TaskGetRequest {}

impl Retrieve for TaskGetRequest {
    async fn select_query(
        &self,
        conn: &mut Object,
        info: Option<Info>,
    ) -> Result<Option<Row>, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Setup
        let info = info.unwrap();

        // Task Get Query
        let params: Vec<&(dyn ToSql + Sync)> = vec![info.user_id(), info.obj_id()];

        let statement = format!(
            "SELECT * FROM todo_data.tasks \
			WHERE {}=$1 AND {}=$2",
            TaskModel::USER_ID,
            TaskModel::ID,
        );

        let row_opt = match transaction.query_opt(&statement, &params).await {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TaskPutRequest {
    title: Option<UpdateMethod<String>>,
    notes: Option<UpdateMethod<String>>,

    start_date: Option<UpdateMethod<NaiveDate>>,
    start_time: Option<UpdateMethod<NaiveTime>>,
    deadline: Option<UpdateMethod<NaiveDate>>,

    area_id: Option<UpdateMethod<Uuid>>,
    project_id: Option<UpdateMethod<Uuid>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    tag_ids: Option<UpdateMethod<Vec<Uuid>>>,
}

impl Update for TaskPutRequest {
    fn columns_vec(&self) -> Vec<String> {
        let mut columns = Vec::new();

        if let Some(_) = &self.title {
            columns.push(TaskModel::TITLE.to_string());
        }
        if let Some(_) = &self.notes {
            columns.push(TaskModel::NOTES.to_string());
        }

        if let Some(_) = &self.start_date {
            columns.push(TaskModel::START_DATE.to_string());
        }
        if let Some(_) = &self.start_time {
            columns.push(TaskModel::START_TIME.to_string());
        }
        if let Some(_) = &self.deadline {
            columns.push(TaskModel::DEADLINE.to_string());
        }

        if let Some(_) = &self.area_id {
            columns.push(TaskModel::AREA_ID.to_string());
        }
        if let Some(_) = &self.project_id {
            columns.push(TaskModel::PROJECT_ID.to_string());
        }

        if let Some(_) = &self.completed {
            columns.push(TaskModel::COMPLETED.to_string());
        }
        if let Some(_) = &self.logged {
            columns.push(TaskModel::LOGGED.to_string());
        }
        if let Some(_) = &self.trashed {
            columns.push(TaskModel::TRASHED.to_string());
        }

        columns
    }

    fn params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

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

    async fn update_query(
        &self,
        conn: &mut Object,
        info: Option<Info>,
    ) -> Result<Option<Row>, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Setup
        let info = info.unwrap();

        // Task Update Query
        let mut params: Vec<&(dyn ToSql + Sync)> = self.params();
        if let Some(b) = self.completed {
            if b {
                params.push(info.timestamp());
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }
        if let Some(b) = self.logged {
            if b {
                params.push(info.timestamp());
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }
        if let Some(b) = self.trashed {
            if b {
                params.push(info.timestamp());
            } else {
                params.push(TIMESTAMP_NULL);
            }
        }
        // TODO: catch if params is empty; return none?
        params.splice(
            0..0,
            [
                info.user_id() as &(dyn ToSql + Sync),
                info.obj_id() as &(dyn ToSql + Sync),
                info.timestamp() as &(dyn ToSql + Sync),
            ],
        );

        let statement = format!(
            "UPDATE todo_data.tasks \
			SET ({}, {})=($3, {}) \
			WHERE {}=$1 AND {}=$2 \
			RETURNING *",
            TaskModel::UPDATED,
            self.columns_vec().join(", "),
            (3..params.len())
                .map(|n| format!("${}", 1 + n))
                .collect::<Vec<String>>()
                .join(", "),
            TaskModel::USER_ID,
            TaskModel::ID,
        );

        let row_opt = match transaction.query_opt(&statement, &params).await {
            Ok(o) => o,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Tag Update Query
        if let (true, Some(tag_update)) = (row_opt.is_some(), &self.tag_ids) {
            let rows_affected = match transaction
                .execute(
                    &format!(
                        "DELETE FROM todo_data.task_tags \
						WHERE {}=$1",
                        TaskTagModel::TASK_ID,
                    ),
                    &[info.obj_id()],
                )
                .await
            {
                Ok(n) => n,
                Err(e) => return Err(Error::DatabaseError(e)),
            };

            if let UpdateMethod::Change(tag_ids) = tag_update {
                let mut params: Vec<&(dyn ToSql + Sync)> =
                    tag_ids.iter().map(|i| i as &(dyn ToSql + Sync)).collect();
                // TODO: catch if params is empty
                params.splice(0..0, [info.obj_id() as &(dyn ToSql + Sync)]);

                let statement = format!(
                    "INSERT INTO todo_data.task_tags \
					({}, {}) \
					VALUES \
					{}",
                    TaskTagModel::TASK_ID,
                    TaskTagModel::TAG_ID,
                    (1..params.len())
                        .map(|n| format!("($1, ${})", 1 + n))
                        .collect::<Vec<String>>()
                        .join(", "),
                );

                let rows_affected = match transaction.execute(&statement, &params).await {
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

#[derive(Debug)]
pub struct TaskDeleteRequest {}

impl Delete for TaskDeleteRequest {
    async fn delete_query(&self, conn: &mut Object, info: Option<Info>) -> Result<bool, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Setup
        let info = info.unwrap();

        // Task Delete Query
        let params: Vec<&(dyn ToSql + Sync)> = vec![info.obj_id()];

        let statement = format!(
            "WITH deleted AS ( \
			DELETE FROM todo_data.task_tags \
			WHERE {0}=$1 \
			) \
			DELETE FROM todo_data.tasks \
			WHERE {0}=$1",
            TaskTagModel::TASK_ID,
        );

        let result = match transaction.execute(&statement, &params).await {
            Ok(0) => false,
            Ok(1) => true,
            Ok(..) => return Err(Error::QueryError("More than one task deleted".to_string())),
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TaskQueryRequest {
    date_filter: Option<DateFilter>,

    area_id: Option<Uuid>,
    project_id: Option<Uuid>,

    tag_ids: Option<Vec<Uuid>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,
}

impl Query for TaskQueryRequest {
    fn conditions(&self) -> Vec<String> {
        let mut conditions = Vec::new();
        let mut n = 2;

        if let Some(f) = &self.date_filter {
            let mut date_conditions: Vec<String> = Vec::new();

            match (&f.start, &f.start_cmp) {
                (_, Some(CmpFlag::IS_NULL)) => {
                    date_conditions.push(format!("{} IS NULL", TaskModel::START_DATE));
                }
                (Some(_), cmp_opt) => {
                    if let Some(cmp) = cmp_opt {
                        date_conditions.push(format!(
                            "{}{}${}",
                            TaskModel::START_DATE,
                            cmp.to_sql_cmp(),
                            n
                        ));
                    } else {
                        date_conditions.push(format!("{}=${}", TaskModel::START_DATE, n));
                    }

                    n += 1;
                }
                _ => (),
            }

            match (&f.deadline, &f.deadline_cmp) {
                (Some(_), Some(CmpFlag::IS_NULL)) => {
                    date_conditions.push(format!("{} IS NULL", TaskModel::DEADLINE));
                }
                (Some(_), cmp_opt) => {
                    if let Some(cmp) = cmp_opt {
                        date_conditions.push(format!(
                            "{}{}${}",
                            TaskModel::DEADLINE,
                            cmp.to_sql_cmp(),
                            n
                        ));
                    } else {
                        date_conditions.push(format!("{}=${}", TaskModel::DEADLINE, n));
                    }

                    n += 1;
                }
                _ => (),
            }

            if f.or {
                conditions.push(date_conditions.join(" OR "));
            } else {
                conditions.push(date_conditions.join(" AND "));
            }
        }

        if let Some(_) = self.area_id {
            conditions.push(format!("{}=${}", TaskModel::AREA_ID, n));

            n += 1;
        }
        if let Some(_) = self.project_id {
            conditions.push(format!("{}=${}", TaskModel::PROJECT_ID, n));
        }

        if let Some(b) = self.completed {
            if b {
                conditions.push(format!("{} IS NOT NULL", TaskModel::COMPLETED));
            } else {
                conditions.push(format!("{} IS NULL", TaskModel::COMPLETED));
            }
        }
        if let Some(b) = self.logged {
            if b {
                conditions.push(format!("{} IS NOT NULL", TaskModel::LOGGED));
            } else {
                conditions.push(format!("{} IS NULL", TaskModel::LOGGED));
            }
        }
        if let Some(b) = self.trashed {
            if b {
                conditions.push(format!("{} IS NOT NULL", TaskModel::TRASHED));
            } else {
                conditions.push(format!("{} IS NULL", TaskModel::TRASHED));
            }
        }

        conditions
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(f) = &self.date_filter {
            match (&f.start, &f.start_cmp) {
                (_, Some(CmpFlag::IS_NULL)) => (),
                (Some(d), _) => params.push(d),
                (_, _) => (),
            }

            match (&f.deadline, &f.deadline_cmp) {
                (_, Some(CmpFlag::IS_NULL)) => (),
                (Some(d), _) => params.push(d),
                (_, _) => (),
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

    async fn query(&self, conn: &mut Object, info: Option<Info>) -> Result<Vec<Row>, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Setup
        let info = info.unwrap();
        let temp: String;

        // Task Query
        let mut params: Vec<&(dyn ToSql + Sync)> = self.params();
        params.splice(0..0, [info.user_id() as &(dyn ToSql + Sync)]);
        let n = params.len();

        let mut statement = format!(
            "SELECT * FROM todo_data.tasks \
			WHERE {}=$1",
            TaskModel::USER_ID,
        );
        let conditions = self.conditions();
        if !conditions.is_empty() {
            statement.push_str(format!(" AND {}", conditions.join(" AND ")).as_str());
        }
        if let Some(query) = info.query() {
            temp = format!("%{}%", query);
            params.push(&temp);
            statement.push_str(format!(" AND {} LIKE ${}", TaskModel::TITLE, n + 1).as_str());
        }

        let mut rows = match transaction.query(&statement, &params).await {
            Ok(r) => r,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Task Tags Query
        if let Some(tag_ids) = &self.tag_ids {
            let params = tag_ids
                .iter()
                .map(|i| i as &(dyn ToSql + Sync))
                .collect::<Vec<&(dyn ToSql + Sync)>>();

            let statement = format!(
                "SELECT task_id FROM todo_data.task_tags \
				WHERE tag_id IN ({}) \
				GROUP BY task_id \
				HAVING COUNT(DISTINCT tag_id) >= {}",
                (0..tag_ids.len())
                    .map(|n| format!("${}", 1 + n))
                    .collect::<Vec<String>>()
                    .join(", "),
                tag_ids.len(),
            );

            let matching_tasks = match transaction.query(&statement, &params).await {
                Ok(r) => r,
                Err(e) => return Err(Error::DatabaseError(e)),
            };

            let task_ids = matching_tasks
                .iter()
                .map(|r| r.get("task_id"))
                .collect::<Vec<Uuid>>();

            rows = rows
                .iter()
                .filter(|r| {
                    let task_id: Uuid = r.get("task_id");

                    task_ids.contains(&task_id)
                })
                .map(|r| r.clone())
                .collect::<Vec<Row>>();
        }

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(rows)
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
