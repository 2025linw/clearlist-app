use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::util::{PostgresCmp, SQLQueryBuilder, ToPostgresCmp, ToSQLQueryBuilder};

use super::{QueryMethod, ToResponse, UpdateMethod};

/// Tag Database Model
#[derive(Debug, Deserialize)]
pub struct DatabaseModel {
    #[serde(alias = "tag_id")]
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
#[derive(Debug, Deserialize, Serialize)]
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
pub struct CreateSchema {
    label: Option<String>,
    color: Option<String>,

    category: Option<String>,
}

impl ToSQLQueryBuilder for CreateSchema {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.set_return_all();

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
#[serde(rename_all = "camelCase")]
pub struct UpdateSchema {
    label: Option<UpdateMethod<String>>,
    color: Option<UpdateMethod<String>>,

    category: Option<UpdateMethod<String>>,

    #[serde(default)]
    timestamp: DateTime<Local>,
}

impl UpdateSchema {
    pub fn is_empty(&self) -> bool {
        self.label.is_none() && self.color.is_none() && self.category.is_none()
    }
}

impl ToSQLQueryBuilder for UpdateSchema {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_column(DatabaseModel::UPDATED, &self.timestamp);
        builder.set_return_all();

        if let Some(ref u) = self.label {
            builder.add_column(DatabaseModel::LABEL, u);
        }
        if let Some(ref u) = self.color {
            builder.add_column(DatabaseModel::COLOR, u);
        }

        if let Some(ref u) = self.category {
            builder.add_column(DatabaseModel::CATEGORY, u);
        }

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct QuerySchema {
    label: Option<QueryMethod<String>>,

    category: Option<QueryMethod<String>>,
}

impl ToSQLQueryBuilder for QuerySchema {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);

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

        builder
    }
}

#[cfg(test)]
mod create_schema_test {
    use crate::util::ToSQLQueryBuilder;

    use super::CreateSchema;

    #[test]
    fn full() {
        let mut schema = CreateSchema::default();
        schema.label = Some("Test Label".to_string());
        schema.color = Some("#2f78ed".to_string());
        schema.category = Some("Priority".to_string());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.tags (tag_label, color, category) VALUES ($1, $2, $3) RETURNING *"
        );
        assert_eq!(params.len(), 3);
    }

    // TEST: make production example
}

#[cfg(test)]
mod update_schema_test {
    use crate::{models::UpdateMethod, util::ToSQLQueryBuilder};

    use super::UpdateSchema;

    #[test]
    fn full() {
        let mut schema = UpdateSchema::default();
        schema.label = Some(UpdateMethod::Set("Test Label".to_string()));
        schema.color = Some(UpdateMethod::Set("#2f78ed".to_string()));
        schema.category = Some(UpdateMethod::Set("Priority".to_string()));

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.tags SET updated_on=$1, tag_label=$2, color=$3, category=$4 RETURNING *"
        );
        assert_eq!(params.len(), 4);
    }

    // TEST: make production example
}

#[cfg(test)]
mod query_schema_test {
    use crate::{models::QueryMethod, util::ToSQLQueryBuilder};

    use super::QuerySchema;

    #[test]
    fn empty() {
        let schema = QuerySchema::default();

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(statement.as_str(), "SELECT * FROM data.tags");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn full() {
        let mut schema = QuerySchema::default();
        schema.label = Some(QueryMethod::Match("Test Label".to_string()));
        schema.category = Some(QueryMethod::Match("Priority".to_string()));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.tags WHERE tag_label ILIKE '%' || $1 || '%' AND category ILIKE '%' || $2 || '%'"
        );
        assert_eq!(params.len(), 2);
    }

    // TEST: make production example
}
