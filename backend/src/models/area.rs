use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::util::{PostgresCmp, SQLQueryBuilder, ToPostgresCmp, ToSQLQueryBuilder};

use super::{QueryMethod, ToResponse, UpdateMethod};

/// Area Database Model
#[derive(Debug, Deserialize)]
pub struct DatabaseModel {
    #[serde(alias = "area_id")]
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
#[derive(Debug, Deserialize, Serialize)]
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
pub struct CreateSchema {
    name: Option<String>,
    icon_url: Option<String>,
}

impl ToSQLQueryBuilder for CreateSchema {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.set_return_all();

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
#[serde(rename_all = "camelCase")]
pub struct UpdateSchema {
    name: Option<UpdateMethod<String>>,
    icon_url: Option<UpdateMethod<String>>,

    #[serde(default)]
    pub(crate) timestamp: DateTime<Local>,
}

impl UpdateSchema {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.icon_url.is_none()
    }
}

impl ToSQLQueryBuilder for UpdateSchema {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        builder.add_column(DatabaseModel::UPDATED, &self.timestamp);
        builder.set_return_all();

        if let Some(ref u) = self.name {
            builder.add_column(DatabaseModel::NAME, u);
        }
        if let Some(ref u) = self.icon_url {
            builder.add_column(DatabaseModel::ICON_URL, u);
        }

        builder
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct QuerySchema {
    name: Option<QueryMethod<String>>,
}

impl ToSQLQueryBuilder for QuerySchema {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(DatabaseModel::TABLE);

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
        schema.name = Some("Test Name".to_string());
        schema.icon_url = Some("https://www.google.com/favicon.ico".to_string());

        let (statement, params) = schema.to_sql_builder().build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.areas (area_name, icon_url) VALUES ($1, $2) RETURNING *"
        );
        assert_eq!(params.len(), 2);
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
        schema.name = Some(UpdateMethod::Set("Test Name".to_string()));
        schema.icon_url = Some(UpdateMethod::Set("https://www.mozilla.org/media/protocol/img/logos/firefox/browser/logo.eb1324e44442.svg".to_string()));

        let (statement, params) = schema.to_sql_builder().build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.areas SET updated_on=$1, area_name=$2, icon_url=$3 RETURNING *"
        );
        assert_eq!(params.len(), 3);
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

        assert_eq!(statement.as_str(), "SELECT * FROM data.areas");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn full() {
        let mut schema = QuerySchema::default();
        schema.name = Some(QueryMethod::Match("Test Name".to_string()));

        let (statement, params) = schema.to_sql_builder().build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.areas WHERE area_name ILIKE '%' || $1 || '%'"
        );
        assert_eq!(params.len(), 1);
    }

    // TEST: make production example
}
