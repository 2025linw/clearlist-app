use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::error::Error;

pub fn extract_user_id(cookies: &CookieJar) -> Result<Uuid, Error> {
    let cookie = match cookies.get("todo_app_user_id") {
        Some(c) => c,
        None => return Err(Error::InvalidRequest("User ID was not sent".to_string())),
    };

    match Uuid::try_parse(cookie.value()) {
        Ok(i) => Ok(i),
        Err(_) => Err(Error::InvalidRequest(
            "User ID was not a UUID format".to_string(),
        )),
    }
}
