use chrono::{DateTime, Local};
use deadpool_postgres::{Object, Transaction};
use serde::Deserialize;
use tokio_postgres::{Row, types::ToSql};
use uuid::Uuid;

use crate::{
    error::Error,
    models::db::{AreaModel, parameter_values},
    storage::db::{DBDelete, DBInsert, DBQuery, DBSelectAll, DBSelectOne, DBUpdate},
};

use super::UpdateMethod;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AreaCreateRequest {
    name: Option<String>,
    icon_url: Option<String>,

    user_id: Option<Uuid>,
}

impl AreaCreateRequest {
    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for AreaCreateRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut columns: Vec<&str> = vec![AreaModel::USER_ID];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        if let Some(s) = &self.name {
            columns.push(AreaModel::NAME);
            params.push(s);
        }
        if let Some(s) = &self.icon_url {
            columns.push(AreaModel::ICON_URL);
            params.push(s);
        }

        let statement = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {} AS id",
            AreaModel::TABLE,
            columns.join(","),
            parameter_values(1..=params.len()).join(","),
            AreaModel::ID,
        );

        (statement, params)
    }
}

impl DBInsert for AreaCreateRequest {
    async fn query(&self, transaction: &Transaction<'_>) -> Result<Row, Error> {
        // Insert Area
        let (statement, params) = self.get_query();

        let row: Row = transaction.query_one(&statement, &params).await?;

        Ok(row)
    }
}

#[derive(Debug, Default)]
pub struct AreaRetrieveRequest {
    area_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl AreaRetrieveRequest {
    pub fn set_area_id(&mut self, id: Uuid) -> &mut Self {
        self.area_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for AreaRetrieveRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![
            self.area_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ];

        let statement = format!(
            "SELECT * FROM {} WHERE {}=$1 AND {}=$2",
            AreaModel::TABLE,
            AreaModel::ID,
            AreaModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBSelectOne for AreaRetrieveRequest {
    async fn query(&self, conn: &Object) -> Result<Option<Row>, Error> {
        // Retrieve Area
        let (statement, params) = self.get_query();

        let row_opt: Option<Row> = conn.query_opt(&statement, &params).await?;

        Ok(row_opt)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AreaUpdateRequest {
    area_id: Option<Uuid>,

    name: Option<UpdateMethod<String>>,
    icon_url: Option<UpdateMethod<String>>,

    user_id: Option<Uuid>,

    #[serde(default = "Local::now")]
    timestamp: DateTime<Local>,
}

impl AreaUpdateRequest {
    pub fn set_area_id(&mut self, id: Uuid) -> &mut Self {
        self.area_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for AreaUpdateRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut updates: Vec<String> = vec![format!("{}=$3", AreaModel::UPDATED)];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![
            self.area_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
            &self.timestamp,
        ];

        let mut n = params.len() + 1;

        let mut update;
        if let Some(u) = &self.name {
            (update, n) = u.update_string(AreaModel::NAME, n);
            updates.push(update);

            if let Some(s) = u.get_param() {
                params.push(s);
            }
        }
        if let Some(u) = &self.icon_url {
            (update, _) = u.update_string(AreaModel::ICON_URL, n);
            updates.push(update);

            if let Some(s) = u.get_param() {
                params.push(s);
            }
        }

        let statement = format!(
            "UPDATE {} SET {} WHERE {}=$1 AND {}=$2 RETURNING *",
            AreaModel::TABLE,
            updates.join(","),
            AreaModel::ID,
            AreaModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBUpdate for AreaUpdateRequest {
    async fn query(&self, transaction: &Transaction<'_>) -> Result<Option<Row>, Error> {
        // Update Area
        let (statement, params) = self.get_query();

        let row_opt: Option<Row> = transaction.query_opt(&statement, &params).await?;

        Ok(row_opt)
    }
}

#[derive(Debug, Default)]
pub struct AreaDeleteRequest {
    area_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl AreaDeleteRequest {
    pub fn set_area_id(&mut self, id: Uuid) -> &mut Self {
        self.area_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for AreaDeleteRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![
            self.area_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ];

        let statement = format!(
            "DELETE FROM {} WHERE {}=$1 AND {}=$2",
            AreaModel::TABLE,
            AreaModel::ID,
            AreaModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBDelete for AreaDeleteRequest {
    async fn query(&self, transaction: &Transaction<'_>) -> Result<bool, Error> {
        // Delete Area
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
pub struct AreaQueryRequest {
    search_query: Option<String>,

    user_id: Option<Uuid>,
}

impl AreaQueryRequest {
    pub fn set_search_query(&mut self, query: String) -> &mut Self {
        self.search_query = Some(format!("%{}%", query));

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for AreaQueryRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut conditions: Vec<String> = vec![format!("{}=$1", AreaModel::USER_ID)];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        // let mut n = conditions.len() + 1; UNCOMMENT IF MORE ENTRIES ARE ADDED
        let n = conditions.len() + 1;

        if let Some(s) = &self.search_query {
            conditions.push(format!("{} LIKE ${}", AreaModel::NAME, n));
            params.push(s);

            // n += 1;
        }

        // let mut condition;

        let statement = format!(
            "SELECT * FROM {} WHERE {}",
            AreaModel::TABLE,
            conditions.join(" AND "),
        );

        (statement, params)
    }
}

impl DBSelectAll for AreaQueryRequest {
    async fn query(&self, conn: &Object) -> Result<Vec<Row>, Error> {
        // Get Areas
        let (statement, params) = self.get_query();

        let rows: Vec<Row> = conn.query(&statement, &params).await?;

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    // use super::*;

    // #[test]
    // fn placeholder() {
    //     unimplemented!()
    // }
}
