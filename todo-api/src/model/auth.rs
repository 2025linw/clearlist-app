use chrono::{DateTime, Local};
use serde::Deserialize;
use uuid::Uuid;

/// User Database Model
#[derive(Debug, Deserialize)]
pub struct UserModel {
    user_id: Uuid,

    email: String,
    password_hash: String,

    created_on: DateTime<Local>,
    updated_on: DateTime<Local>,
}

impl UserModel {
    pub const TABLE: &str = "auth.users";

    pub const ID: &str = "user_id";

    pub const EMAIL: &str = "email";
    pub const PASS_HASH: &str = "password_hash";

    pub const CREATED: &str = "created_on";
    pub const UPDATED: &str = "updated_on";
}
