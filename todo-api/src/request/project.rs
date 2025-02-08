use deadpool_postgres::Object;
use serde::Deserialize;
use tokio_postgres::{types::ToSql, Row};

use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::{
    database::{ProjectModel, ProjectTagModel, PROJECT_TABLE},
    request::{
        api::{Create, Delete, Query, Retrieve, Update},
        query::{CmpFlag, ToQuery, TIMESTAMP_NULL},
        UpdateMethod,
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

    user_id: Option<Uuid>,

    tag_ids: Option<Vec<Uuid>>,
}

impl ProjectPostRequest {
    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Create for ProjectPostRequest {
    async fn query(&self, conn: &mut Object) -> Result<Row, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Project Insert Query
        let row = match transaction
            .query_one(&self.statement(), &self.params())
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Project Tag Insert Query
        if let Some(tag_ids) = &self.tag_ids {
            todo!();
        }

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(row)
    }
}

impl ToQuery for ProjectPostRequest {
    fn statement(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ProjectGetRequest {
    project_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl ProjectGetRequest {
    pub fn new() -> Self {
        Self {
            project_id: None,

            user_id: None,
        }
    }

    pub fn project_id(&mut self, id: Uuid) -> &mut Self {
        self.project_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Retrieve for ProjectGetRequest {
    async fn query(&self, conn: &mut Object) -> Result<Option<Row>, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Project Get Query
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

impl ToQuery for ProjectGetRequest {
    fn statement(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProjectPutRequest {
    project_id: Option<Uuid>,

    title: Option<UpdateMethod<String>>,
    notes: Option<UpdateMethod<String>>,

    start_date: Option<UpdateMethod<NaiveDate>>,
    start_time: Option<UpdateMethod<NaiveTime>>,
    deadline: Option<UpdateMethod<NaiveDate>>,

    area_id: Option<UpdateMethod<Uuid>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    user_id: Option<Uuid>,

    tag_id: Option<UpdateMethod<Vec<Uuid>>>,
}

impl ProjectPutRequest {
    pub fn project_id(&mut self, id: Uuid) -> &mut Self {
        self.project_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Update for ProjectPutRequest {
    async fn query(&self, conn: &mut Object) -> Result<Option<Row>, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Project Update Query
        let row_opt = match transaction
            .query_opt(&self.statement(), &self.params())
            .await
        {
            Ok(o) => o,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Project Tag Update Query
        if let (true, Some(tag_update)) = (row_opt.is_some(), &self.tag_id) {
            todo!();
        }

        // Commit
        if let Err(e) = transaction.commit().await {
            return Err(Error::DatabaseError(e));
        }

        Ok(row_opt)
    }
}

impl ToQuery for ProjectPutRequest {
    fn statement(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ProjectDeleteRequest {
    project_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl ProjectDeleteRequest {
    pub fn new() -> Self {
        Self {
            project_id: None,

            user_id: None,
        }
    }

    pub fn project_id(&mut self, id: Uuid) -> &mut Self {
        self.project_id = Some(id);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Delete for ProjectDeleteRequest {
    async fn query(&self, conn: &mut Object) -> Result<bool, Error> {
        let transaction = match conn.transaction().await {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(e)),
        };

        // Project Delete Query
        let result = match transaction.execute(&self.statement(), &self.params()).await {
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

impl ToQuery for ProjectDeleteRequest {
    fn statement(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProjectQueryRequest {
    user_id: Option<Uuid>,

    search_query: Option<String>,

    start_date: Option<(NaiveDate, CmpFlag)>,
    deadline: Option<(NaiveDate, CmpFlag)>,

    area_id: Option<(Uuid, CmpFlag)>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    tag_ids: Option<(Vec<Uuid>, CmpFlag)>,
}

impl ProjectQueryRequest {
    pub fn search_query(&mut self, query: String) -> &mut Self {
        self.search_query = Some(query);

        self
    }

    pub fn user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl Query for ProjectQueryRequest {
    async fn query(&self, conn: &mut Object) -> Result<Vec<Row>, Error> {
        todo!()
    }
}

impl ToQuery for ProjectQueryRequest {
    fn statement(&self) -> String {
        todo!()
    }

    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
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
