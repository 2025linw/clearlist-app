pub mod token;
pub mod user;

use serde::{Deserialize, Serialize};

use token::ResponseModel as TokenResponseModel;
use user::ResponseModel as UserResponseModel;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct ResetSchema {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenResponseModel {
    #[serde(flatten)]
    user: UserResponseModel,
    #[serde(flatten)]
    token: TokenResponseModel,
}

impl UserTokenResponseModel {
    pub fn new(user: UserResponseModel, token: TokenResponseModel) -> Self {
        Self { user, token }
    }
}
