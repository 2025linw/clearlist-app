use serde::Deserialize;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::{
    models::user,
    util::{SqlQueryBuilder, ToSqlQueryBuilder},
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
    email: String,
    password_hash: String,
}

impl LoginInfo {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            email,
            password_hash,
        }
    }
}

impl ToSqlQueryBuilder for LoginInfo {
    fn to_sql_builder(&self) -> SqlQueryBuilder {
        let mut builder = SqlQueryBuilder::new(user::DatabaseModel::TABLE);
        builder.add_column(user::DatabaseModel::EMAIL, &self.email);
        builder.add_column(user::DatabaseModel::PASS_HASH, &self.password_hash);

        builder.set_return(&[user::DatabaseModel::EMAIL, user::DatabaseModel::PASS_HASH]);

        builder
    }
}

pub struct UserLogin {
    user_id: Uuid,
    _email: String, // TODO: do we need this?
    password_hash: String,
}

impl UserLogin {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}

impl From<Row> for UserLogin {
    fn from(value: Row) -> Self {
        Self {
            user_id: value.get(user::DatabaseModel::ID),
            _email: value.get(user::DatabaseModel::EMAIL),
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
