use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::util::{PostgresCmp, SQLQueryBuilder, ToPostgresCmp, ToSQLQueryBuilder};

use super::{QueryMethod, ToResponse, UpdateMethod};

/// Tag Database Model
#[derive(Debug)]
pub struct DatabaseModel {
    id: Uuid,

    tag_label: Option<String>,
    color: Option<String>,

    category: Option<String>,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl DatabaseModel {
    pub const TABLE: &str = "data.tags";

    pub const ID: &str = "tag_id";

    pub const LABEL: &str = "tag_label";
    pub const COLOR: &str = "color";

    pub const CATEGORY: &str = "category";

    pub const USER_ID: &str = "user_id";
    pub const CREATED: &str = "created_on";
    pub const UPDATED: &str = "updated_on";
}

impl From<Row> for DatabaseModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get(Self::ID),
            tag_label: value.get(Self::LABEL),
            color: value.get(Self::COLOR),
            category: value.get(Self::CATEGORY),
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
            label: self.tag_label.to_owned().unwrap_or_default(),
            color: self.color.to_owned().unwrap_or_default(),
            category: self.category.to_owned().unwrap_or_default(),
            user_id: self.user_id,
            created_on: self.created_on,
            updated_on: self.updated_on,
        }
    }
}

/// Tag Response Model
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseModel {
    id: Uuid,

    label: String,
    color: String,

    category: String,

    user_id: Uuid,
    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    label: Option<String>,
    color: Option<String>,

    category: Option<String>,

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

        if let Some(ref s) = self.label {
            builder.add_column(DatabaseModel::LABEL, s);
        }
        if let Some(ref s) = self.color {
            builder.add_column(DatabaseModel::COLOR, s);
        }

        if let Some(ref s) = self.category {
            builder.add_column(DatabaseModel::CATEGORY, s);
        }

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct RetrieveRequest {
    tag_id: Uuid,

    #[serde(skip)]
    user_id: Uuid,
}

impl RetrieveRequest {
    pub fn new(tag_id: Uuid, user_id: Uuid) -> Self {
        Self { tag_id, user_id }
    }

    pub fn is_valid(&self) -> bool {
        self.tag_id != Uuid::default() && self.user_id != Uuid::default()
    }
}

impl ToSQLQueryBuilder for RetrieveRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &self.user_id);

        builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &self.tag_id);

        builder.set_return_all();

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    label: Option<UpdateMethod<String>>,
    color: Option<UpdateMethod<String>>,

    category: Option<UpdateMethod<String>>,

    #[serde(default = "chrono::Local::now")]
    timestamp: DateTime<Local>,

    #[serde(skip)]
    tag_id: Uuid,
    #[serde(skip)]
    user_id: Uuid,
}

impl UpdateRequest {
    pub fn is_empty(&self) -> bool {
        self.label.is_none() && self.color.is_none() && self.category.is_none()
    }

    pub fn is_valid(&self) -> bool {
        !self.is_empty() && self.tag_id != Uuid::default() && self.user_id != Uuid::default()
    }

    pub fn set_tag_id(&mut self, tag_id: Uuid) {
        self.tag_id = tag_id;
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

        if let Some(ref u) = self.label {
            builder.add_column(DatabaseModel::LABEL, u);
        }
        if let Some(ref u) = self.color {
            builder.add_column(DatabaseModel::COLOR, u);
        }

        if let Some(ref u) = self.category {
            builder.add_column(DatabaseModel::CATEGORY, u);
        }

        builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &self.user_id);
        builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &self.tag_id);

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DeleteRequest {
    tag_id: Uuid,

    #[serde(skip)]
    user_id: Uuid,
}

impl DeleteRequest {
    pub fn new(tag_id: Uuid, user_id: Uuid) -> Self {
        Self { tag_id, user_id }
    }

    pub fn set_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }

    pub fn is_valid(&self) -> bool {
        self.tag_id != Uuid::default() && self.user_id != Uuid::default()
    }
}

impl ToSQLQueryBuilder for DeleteRequest {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &self.user_id);

        builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &self.tag_id);

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    label: Option<QueryMethod<String>>,

    category: Option<QueryMethod<String>>,

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

        if let Some(ref q) = self.label {
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
            builder.add_condition(DatabaseModel::LABEL, cmp, q);
        }

        if let Some(ref q) = self.category {
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
            builder.add_condition(DatabaseModel::CATEGORY, cmp, q);
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
        schema.label = Some("Test Label".to_string());
        schema.color = Some("#2f78ed".to_string());
        schema.category = Some("Priority".to_string());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.tags (user_id, tag_label, color, category) VALUES ($1, $2, $3, $4) RETURNING tag_id"
        );
        assert_eq!(params.len(), 4);
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
            "SELECT * FROM data.tags WHERE user_id = $1 AND tag_id = $2"
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
        schema.label = Some(UpdateMethod::Set("Test Label".to_string()));
        schema.color = Some(UpdateMethod::Set("#2f78ed".to_string()));
        schema.category = Some(UpdateMethod::Set("Priority".to_string()));

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tags SET updated_on=$1, tag_label=$2, color=$3, category=$4 WHERE user_id = $5 AND tag_id = $6 RETURNING tag_id"
        );
        assert_eq!(params.len(), 6);
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
            "DELETE FROM data.tags WHERE user_id = $1 AND tag_id = $2"
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
            "SELECT * FROM data.tags WHERE user_id = $1 LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn full() {
        let mut schema = QueryRequest::default();
        schema.label = Some(QueryMethod::Match("Test Label".to_string()));
        schema.category = Some(QueryMethod::Match("Priority".to_string()));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tags WHERE user_id = $1 AND tag_label ILIKE '%' || $2 || '%' AND category ILIKE '%' || $3 || '%' LIMIT 25 OFFSET 0"
        );
        assert_eq!(params.len(), 3);
    }
}
