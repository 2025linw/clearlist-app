use chrono::{DateTime, Local};
use deadpool_postgres::{Object, Transaction};
use serde::Deserialize;
use tokio_postgres::{Row, types::ToSql};
use uuid::Uuid;

use crate::{
    error::Error,
    storage::{
        db::{DBDelete, DBInsert, DBQuery, DBSelectAll, DBSelectOne, DBUpdate},
        model::TagModel,
    },
    util::parameter_values,
};

use super::{QueryMethod, UpdateMethod};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TagCreateRequest {
    label: Option<String>,
    category: Option<String>,
    color: Option<String>,

    user_id: Option<Uuid>,
}

impl TagCreateRequest {
    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TagCreateRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut columns: Vec<&str> = vec![TagModel::USER_ID];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        if let Some(s) = &self.label {
            columns.push(TagModel::LABEL);
            params.push(s);
        }
        if let Some(s) = &self.category {
            columns.push(TagModel::CATEGORY);
            params.push(s);
        }
        if let Some(c) = &self.color {
            columns.push(TagModel::COLOR);
            params.push(c);
        }

        let statement = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {} AS id",
            TagModel::TABLE,
            columns.join(","),
            parameter_values(1..=params.len()).join(","),
            TagModel::ID,
        );

        (statement, params)
    }
}

impl DBInsert for TagCreateRequest {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<Row, Error> {
        // Insert Tag
        let (statement, params) = self.get_query();

        let row: Row = transaction.query_one(&statement, &params).await?;

        Ok(row)
    }
}

#[derive(Debug, Default)]
pub struct TagRetrieveRequest {
    tag_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl TagRetrieveRequest {
    pub fn set_tag_id(&mut self, id: Uuid) -> &mut Self {
        self.tag_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TagRetrieveRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![
            self.tag_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ];

        let statement = format!(
            "SELECT * FROM {} WHERE {}=$1 AND {}=$2",
            TagModel::TABLE,
            TagModel::ID,
            TagModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBSelectOne for TagRetrieveRequest {
    async fn query(&self, conn: &Object) -> Result<Option<Row>, Error> {
        // Retrieve Tag
        let (statement, params) = self.get_query();

        let row_opt: Option<Row> = conn.query_opt(&statement, &params).await?;

        Ok(row_opt)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TagUpdateRequest {
    tag_id: Option<Uuid>,

    label: Option<UpdateMethod<String>>,
    category: Option<UpdateMethod<String>>,
    color: Option<UpdateMethod<String>>,

    user_id: Option<Uuid>,

    #[serde(default = "Local::now")]
    timestamp: DateTime<Local>,
}

impl TagUpdateRequest {
    pub fn set_tag_id(&mut self, id: Uuid) -> &mut Self {
        self.tag_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TagUpdateRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut columns: Vec<&str> = vec![TagModel::UPDATED];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![
            self.tag_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
            &self.timestamp,
        ];

        if let Some(u) = &self.label {
            columns.push(TagModel::LABEL);
            if let Some(s) = u.get_param() {
                params.push(s);
            }
        }
        if let Some(u) = &self.category {
            columns.push(TagModel::CATEGORY);
            if let Some(s) = u.get_param() {
                params.push(s);
            }
        }
        if let Some(u) = &self.color {
            columns.push(TagModel::COLOR);
            if let Some(c) = u.get_param() {
                params.push(c);
            }
        }

        let statement = format!(
            "UPDATE {} SET ({})=({}) WHERE {}=$1 AND {}=$2 RETURNING *",
            TagModel::TABLE,
            columns.join(","),
            (0..params.len())
                .map(|n| format!("${}", 3 + n))
                .collect::<Vec<String>>()
                .join(","),
            TagModel::ID,
            TagModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBUpdate for TagUpdateRequest {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<Option<Row>, Error> {
        // Update Tag
        let (statement, params) = self.get_query();

        let row_opt: Option<Row> = transaction.query_opt(&statement, &params).await?;

        Ok(row_opt)
    }
}

#[derive(Debug, Default)]
pub struct TagDeleteRequest {
    tag_id: Option<Uuid>,

    user_id: Option<Uuid>,
}

impl TagDeleteRequest {
    pub fn set_tag_id(&mut self, id: Uuid) -> &mut Self {
        self.tag_id = Some(id);

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TagDeleteRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let params: Vec<&(dyn ToSql + Sync)> = vec![
            self.tag_id.as_ref().unwrap(),
            self.user_id.as_ref().unwrap(),
        ];

        let statement = format!(
            "DELETE FROM {} WHERE {}=$1 AND {}=$2 RETURNING *",
            TagModel::TABLE,
            TagModel::ID,
            TagModel::USER_ID,
        );

        (statement, params)
    }
}

impl DBDelete for TagDeleteRequest {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<bool, Error> {
        // Delete Tag
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
pub struct TagQueryRequest {
    search_query: Option<String>,

    category: Option<QueryMethod<String>>,

    user_id: Option<Uuid>,
}

impl TagQueryRequest {
    pub fn set_search_query(&mut self, query: String) -> &mut Self {
        self.search_query = Some(format!("%{}%", query));

        self
    }

    pub fn set_user_id(&mut self, id: Uuid) -> &mut Self {
        self.user_id = Some(id);

        self
    }
}

impl DBQuery for TagQueryRequest {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut conditions: Vec<String> = vec![format!("{}=$1", TagModel::USER_ID)];
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![self.user_id.as_ref().unwrap()];

        let mut n = conditions.len() + 1;

        if let Some(s) = &self.search_query {
            conditions.push(format!("{} LIKE ${}", TagModel::LABEL, n));
            params.push(s);

            n += 1;
        }

        let condition;
        // let mut condition;
        if let Some(q) = &self.category {
            (condition, _) = q.condition_string(TagModel::CATEGORY, n);
            conditions.push(condition);

            if let Some(s) = q.get_param() {
                params.push(s);
            }
        }

        let statement = format!(
            "SELECT * FROM {} WHERE {}",
            TagModel::TABLE,
            conditions.join(" AND "),
        );

        (statement, params)
    }
}

impl DBSelectAll for TagQueryRequest {
    async fn query(&self, conn: &Object) -> Result<Vec<Row>, Error> {
        // Get Tags
        let (statement, params) = self.get_query();

        let rows = conn.query(&statement, &params).await?;

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn placeholder() {
        unimplemented!()
    }
}
