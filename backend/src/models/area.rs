use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::util::{PostgresCmp, SQLQueryBuilder, ToPostgresCmp, ToSQLQueryBuilder};

use super::{QueryMethod, ToResponse, UpdateMethod};

/// Area Database Model
#[derive(Debug)]
pub struct DatabaseModel {
    id: Uuid,

    area_name: Option<String>,
    icon_url: Option<String>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl DatabaseModel {
    pub const TABLE: &str = "data.areas";

    pub const ID: &str = "area_id";

    pub const NAME: &str = "area_name";
    pub const ICON_URL: &str = "icon_url";

    pub const USER_ID: &str = "user_id";
    pub const CREATED: &str = "created_on";
    pub const UPDATED: &str = "updated_on";
}

impl From<Row> for DatabaseModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get(Self::ID),
            area_name: value.get(Self::NAME),
            icon_url: value.get(Self::ICON_URL),
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
            name: self.area_name.to_owned().unwrap_or_default(),
            icon_url: self.icon_url.to_owned().unwrap_or_default(),
            user_id: self.user_id,
            created_on: self.created_on,
            updated_on: self.updated_on,
        }
    }
}

/// Area Response Model
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseModel {
    id: Uuid,

    name: String,
    icon_url: String,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    name: Option<String>,
    icon_url: Option<String>,

    #[serde(skip)]
    user_id: Uuid,
}

impl CreateRequest {
    pub fn set_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }

    pub fn is_valid(&self) -> bool {
        self.user_id != Uuid::default()
    }
}

impl ToSQLQueryBuilder for CreateRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_column(DatabaseModel::USER_ID, &self.user_id);
        builder.set_return(&[DatabaseModel::ID]);

        if let Some(ref s) = self.name {
            builder.add_column(DatabaseModel::NAME, s);
        }
        if let Some(ref s) = self.icon_url {
            builder.add_column(DatabaseModel::ICON_URL, s);
        }

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct RetrieveRequest {
    area_id: Uuid,

    #[serde(skip)]
    user_id: Uuid,
}

impl RetrieveRequest {
    pub fn new(area_id: Uuid, user_id: Uuid) -> Self {
        Self { area_id, user_id }
    }

    pub fn is_valid(&self) -> bool {
        self.area_id != Uuid::default() && self.user_id != Uuid::default()
    }
}

impl ToSQLQueryBuilder for RetrieveRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &self.user_id);

        builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &self.area_id);

        builder.set_return_all();

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    name: Option<UpdateMethod<String>>,
    icon_url: Option<UpdateMethod<String>>,

    #[serde(default = "chrono::Local::now")]
    timestamp: DateTime<Local>,

    #[serde(skip)]
    area_id: Uuid,
    #[serde(skip)]
    user_id: Uuid,
}

impl UpdateRequest {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.icon_url.is_none()
    }

    pub fn is_valid(&self) -> bool {
        !self.is_empty() && self.area_id != Uuid::default() && self.user_id != Uuid::default()
    }

    pub fn set_area_id(&mut self, area_id: Uuid) {
        self.area_id = area_id;
    }

    pub fn set_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }
}

impl ToSQLQueryBuilder for UpdateRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_column(DatabaseModel::UPDATED, &self.timestamp);
        builder.set_return(&[DatabaseModel::ID]);

        if let Some(ref u) = self.name {
            builder.add_column(DatabaseModel::NAME, u);
        }
        if let Some(ref u) = self.icon_url {
            builder.add_column(DatabaseModel::ICON_URL, u);
        }

        builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &self.user_id);
        builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &self.area_id);

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DeleteRequest {
    area_id: Uuid,

    #[serde(skip)]
    user_id: Uuid,
}

impl DeleteRequest {
    pub fn new(area_id: Uuid, user_id: Uuid) -> Self {
        Self { area_id, user_id }
    }

    pub fn set_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }

    pub fn is_valid(&self) -> bool {
        self.area_id != Uuid::default() && self.user_id != Uuid::default()
    }
}

impl ToSQLQueryBuilder for DeleteRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &self.user_id);

        builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &self.area_id);

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    name: Option<QueryMethod<String>>,

    limit: Option<usize>,
    offset: Option<usize>,

    #[serde(skip)]
    user_id: Uuid,
}

impl QueryRequest {
    pub fn set_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = Some(offset);
    }

    pub fn is_valid(&self) -> bool {
        self.user_id != Uuid::default()
    }
}

impl ToSQLQueryBuilder for QueryRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &self.user_id);
        builder.set_return_all();

        if let Some(ref q) = self.name {
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
            builder.add_condition(DatabaseModel::NAME, cmp, q);
        }

        builder.set_limit(self.limit.unwrap_or(25));
        builder.set_offset(self.offset.unwrap_or(0));

        builder
    }
}

#[cfg(test)]
mod create_schema {
    use crate::util::ToSQLQueryBuilder;

    use super::CreateRequest;

    #[test]
    fn full() {
        let mut schema = CreateRequest::default();
        schema.name = Some("Test Name".to_string());
        schema.icon_url = Some("https://www.google.com/favicon.ico".to_string());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.areas (user_id, area_name, icon_url) VALUES ($1, $2, $3) RETURNING area_id"
        );
        assert_eq!(params.len(), 3);
    }
}

#[cfg(test)]
mod retrieve_schema {
    use uuid::Uuid;

    use crate::util::ToSQLQueryBuilder;

    use super::RetrieveRequest;

    #[test]
    fn is_valid() {
        let schema = RetrieveRequest::default();

        assert!(!schema.is_valid());

        let schema = RetrieveRequest::new(Uuid::new_v4(), Uuid::nil());

        assert!(!schema.is_valid());

        let schema = RetrieveRequest::new(Uuid::nil(), Uuid::new_v4());

        assert!(!schema.is_valid());

        let schema = RetrieveRequest::new(Uuid::new_v4(), Uuid::new_v4());

        assert!(schema.is_valid());
    }

    #[test]
    fn full() {
        let schema = RetrieveRequest::new(Uuid::nil(), Uuid::nil());

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement,
            "SELECT * FROM data.areas WHERE user_id = $1 AND area_id = $2"
        );
        assert_eq!(params.len(), 2)
    }
}

#[cfg(test)]
mod update_schema {
    use crate::{models::UpdateMethod, util::ToSQLQueryBuilder};

    use super::UpdateRequest;

    #[test]
    fn full() {
        let mut schema = UpdateRequest::default();
        schema.name = Some(UpdateMethod::Set("Test Name".to_string()));
        schema.icon_url = Some(UpdateMethod::Set("https://www.mozilla.org/media/protocol/img/logos/firefox/browser/logo.eb1324e44442.svg".to_string()));

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.areas SET updated_on=$1, area_name=$2, icon_url=$3 WHERE user_id = $4 AND area_id = $5 RETURNING area_id"
        );
        assert_eq!(params.len(), 5);
    }
}

#[cfg(test)]
mod delete_schema {
    use uuid::Uuid;

    use crate::util::ToSQLQueryBuilder;

    use super::DeleteRequest;

    #[test]
    fn is_valid() {
        let schema = DeleteRequest::default();

        assert!(!schema.is_valid());

        let schema = DeleteRequest::new(Uuid::new_v4(), Uuid::nil());

        assert!(!schema.is_valid());

        let schema = DeleteRequest::new(Uuid::nil(), Uuid::new_v4());

        assert!(!schema.is_valid());

        let schema = DeleteRequest::new(Uuid::new_v4(), Uuid::new_v4());

        assert!(schema.is_valid());
    }

    #[test]
    fn full() {
        let schema = DeleteRequest::new(Uuid::nil(), Uuid::nil());

        let (statement, params) = schema.to_sql_builder().build_delete();

        assert_eq!(
            statement,
            "DELETE FROM data.areas WHERE user_id = $1 AND area_id = $2"
        );
        assert_eq!(params.len(), 2)
    }
}

#[cfg(test)]
mod query_schema {
    use crate::{models::QueryMethod, util::ToSQLQueryBuilder};

    use super::QueryRequest;

    #[test]
    fn empty() {
        let schema = QueryRequest::default();

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.areas WHERE user_id = $1 LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn full() {
        let mut schema = QueryRequest::default();
        schema.name = Some(QueryMethod::Match("Test Name".to_string()));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.areas WHERE user_id = $1 AND area_name ILIKE '%' || $2 || '%' LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 2);
    }
}
