use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseModel {
    access_jwt: String,
    refresh_jwt: Option<String>,
}

impl ResponseModel {
    pub fn new(access_jwt: String) -> Self {
        Self {
            access_jwt,
            refresh_jwt: None,
        }
    }

    pub fn set_refresh_jwt(&mut self, refresh_jwt: String) {
        self.refresh_jwt = Some(refresh_jwt);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claim {
    pub iss: String,
    #[serde(with = "uuid::serde::simple")]
    pub sub: Uuid,
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
}

impl Claim {
    pub fn new(sub: Uuid, exp: u64) -> Self {
        Self {
            sub,
            exp,
            ..Default::default()
        }
    }
}

impl Default for Claim {
    fn default() -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(1);

        Self {
            iss: "todo-app-auth".to_string(),
            aud: "todo-app-user".to_string(),
            sub: Uuid::nil(),
            exp: exp.timestamp() as u64,
            iat: iat.timestamp() as u64,
        }
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub refresh_jwt: String,
}
