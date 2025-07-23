use serde::Deserialize;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::{
    models::user,
    util::{SQLQueryBuilder, ToSQLQueryBuilder},
};

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    email: String,
    password: String,
}

impl LoginRequest {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    user_id: Uuid,
    email: String,
    password_hash: String,
}

impl LoginInfo {
    pub fn from_request(request: LoginRequest, password_hash: String) -> Self {
        Self {
            user_id: Uuid::nil(),
            email: request.email,
            password_hash,
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}

impl ToSQLQueryBuilder for LoginInfo {
    fn to_sql_builder(&self) -> SQLQueryBuilder {
        let mut builder = SQLQueryBuilder::new(user::DatabaseModel::TABLE);
        builder.add_column(user::DatabaseModel::EMAIL, &self.email);
        builder.add_column(user::DatabaseModel::PASS_HASH, &self.password_hash);

        builder.set_return(&[user::DatabaseModel::EMAIL, user::DatabaseModel::PASS_HASH]);

        builder
    }
}

impl From<Row> for LoginInfo {
    fn from(value: Row) -> Self {
        Self {
            user_id: value.get(user::DatabaseModel::ID),
            email: value.get(user::DatabaseModel::EMAIL),
            password_hash: value.get(user::DatabaseModel::PASS_HASH),
        }
    }
}

// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(Default))]
// #[serde(rename_all = "camelCase")]
// pub struct ResetSchema {
//     pub email: String,
// }
