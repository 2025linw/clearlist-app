use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LoginDetails {
    pub email: Option<String>,
    pub password: Option<String>,
}
