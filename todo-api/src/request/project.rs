use deadpool_postgres::Object;
use serde::Deserialize;
use tokio_postgres::{types::ToSql, Row};

use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::{
    database::{ProjectModel, ProjectTagModel},
    request::{
        api::{Create, Delete, Info, Query, Retrieve, Update},
        query::CmpFlag,
        DateFilter, UpdateMethod, TIMESTAMP_NULL,
    },
    response::Error,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProjectPostRequest {
    title: Option<String>,
    notes: Option<String>,

    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,

    tag_ids: Option<Vec<Uuid>>,
}

impl Create for ProjectPostRequest {
    fn columns_vec(&self) -> Vec<String> {
        let mut columns: Vec<String> = Vec::new();

        if self.title.is_some() {
            columns.push(ProjectModel::TITLE.to_string());
        }
        if self.notes.is_some() {
            columns.push(ProjectModel::NOTES.to_string());
        }

        if self.start_date.is_some() {
            columns.push(ProjectModel::START_DATE.to_string());
        }
        if self.start_time.is_some() {
            columns.push(ProjectModel::START_TIME.to_string());
        }
        if self.deadline.is_some() {
            columns.push(ProjectModel::DEADLINE.to_string());
        }

        if self.area_id.is_some() {
            columns.push(ProjectModel::AREA_ID.to_string());
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

        params
    }

    async fn insert_query(&self, conn: &mut Object, info: Option<Info>) -> Result<Row, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Setup
        let info = info.unwrap();

        // Project Insert Query
        let mut params = self.params();
        params.splice(0..0, [info.user_id() as &(dyn ToSql + Sync)]);

        let statement = format!(
            "INSERT INTO todo_data.projects \
			({}, {}) \
			VALUES \
			($1, {}) \
			RETURNING task_id",
            ProjectModel::USER_ID,
            self.columns_vec().join(", "),
            (1..params.len()) // TODO: make a function or macro to prevent rewriting this
                .map(|n| format!("${}", 1 + n))
                .collect::<Vec<String>>()
                .join(", "),
        );

        let row = match transaction.query_one(&statement, &params).await {
            Ok(r) => r,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Project Tag Insert Query
        if let Some(tag_ids) = &self.tag_ids {
            let project_id: Uuid = row.get(ProjectModel::ID);

            let mut params: Vec<&(dyn ToSql + Sync)> = vec![&project_id];
            for tag_id in tag_ids {
                params.push(tag_id);
            }

            let statement = format!(
                "INSERT INTO todo_data.project_tags \
				({}, {}) \
				VALUES \
				{}",
                ProjectTagModel::PROJECT_ID,
                ProjectTagModel::TAG_ID,
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
pub struct ProjectGetRequest {}

impl Retrieve for ProjectGetRequest {
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

        // Project Get Query
        let params: Vec<&(dyn ToSql + Sync)> = vec![info.user_id(), info.obj_id()];

        let statement = format!(
            "SELECT * FROM todo_data.projects \
			WHERE {}=$1 AND {}=$2",
            ProjectModel::USER_ID,
            ProjectModel::ID,
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
pub struct ProjectPutRequest {
    title: Option<UpdateMethod<String>>,
    notes: Option<UpdateMethod<String>>,

    start_date: Option<UpdateMethod<NaiveDate>>,
    start_time: Option<UpdateMethod<NaiveTime>>,
    deadline: Option<UpdateMethod<NaiveDate>>,

    area_id: Option<UpdateMethod<Uuid>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    tag_id: Option<UpdateMethod<Vec<Uuid>>>,
}

impl Update for ProjectPutRequest {
    fn columns_vec(&self) -> Vec<String> {
        let mut columns = Vec::new();

        if let Some(_) = &self.title {
            columns.push(ProjectModel::TITLE.to_string());
        }
        if let Some(_) = &self.notes {
            columns.push(ProjectModel::NOTES.to_string());
        }

        if let Some(_) = &self.start_date {
            columns.push(ProjectModel::START_DATE.to_string());
        }
        if let Some(_) = &self.start_time {
            columns.push(ProjectModel::START_TIME.to_string());
        }
        if let Some(_) = &self.deadline {
            columns.push(ProjectModel::DEADLINE.to_string());
        }

        if let Some(_) = &self.area_id {
            columns.push(ProjectModel::AREA_ID.to_string());
        }

        if let Some(_) = &self.completed {
            columns.push(ProjectModel::COMPLETED.to_string());
        }
        if let Some(_) = &self.logged {
            columns.push(ProjectModel::LOGGED.to_string());
        }
        if let Some(_) = &self.trashed {
            columns.push(ProjectModel::TRASHED.to_string());
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

        // Project Update Query
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
            "UPDATE todo_data.projects \
			SET ({}, {})=($3, {}) \
			WHERE {}=$1 AND {}=$2 \
			RETURNING *",
            ProjectModel::UPDATED,
            self.columns_vec().join(", "),
            (3..params.len())
                .map(|n| format!("${}", 1 + n))
                .collect::<Vec<String>>()
                .join(", "),
            ProjectModel::USER_ID,
            ProjectModel::ID,
        );

        let row_opt = match transaction.query_opt(&statement, &params).await {
            Ok(o) => o,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Project Tag Update Query
        if let (true, Some(tag_update)) = (row_opt.is_some(), &self.tag_id) {
            let rows_affected = match transaction
                .execute(
                    &format!(
                        "DELETE FROM todo_data.project_tags \
						WHERE {}=$1",
                        ProjectTagModel::PROJECT_ID,
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
                    ProjectTagModel::PROJECT_ID,
                    ProjectTagModel::TAG_ID,
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
pub struct ProjectDeleteRequest {}

impl Delete for ProjectDeleteRequest {
    async fn delete_query(&self, conn: &mut Object, info: Option<Info>) -> Result<bool, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Setup
        let info = info.unwrap();

        // Project Delete Query
        let params: Vec<&(dyn ToSql + Sync)> = vec![info.obj_id()];

        let statement = format!(
            "WITH deleted AS ( \
			DELETE FROM todo_data.project_tags \
			WHERE {0}=$1 \
			) \
			DELETE FROM todo_data.projects \
			WHERE {0}=$1",
            ProjectTagModel::PROJECT_ID,
        );

        let result = match transaction.execute(&statement, &params).await {
            Ok(0) => false,
            Ok(1) => true,
            Ok(..) => {
                return Err(Error::QueryError(
                    "More than one project deleted".to_string(),
                ))
            }
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
pub struct ProjectQueryRequest {
    data_filter: Option<DateFilter>,

    area_id: Option<Uuid>,

    tag_ids: Option<Vec<Uuid>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    user_id: Uuid,
}

impl Query for ProjectQueryRequest {
    fn conditions(&self) -> Vec<String> {
        todo!()
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        todo!()
    }

    async fn query(&self, conn: &mut Object, info: Option<Info>) -> Result<Vec<Row>, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // TODO: Project request testing
    // Directly call with premade Requests
    use super::*;

    #[test]
    fn test_add() {
        assert!(true);
    }
}
