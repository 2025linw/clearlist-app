use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Duration;
use serde_json::json;

use crate::{
    AppState,
    data::{check_for_email, check_for_user, get_login_info, register_user},
    error::{ErrorResponse, LOGIN_EXISTS, LOGIN_FAILED},
    models::{
        auth::{
            LoginInfo,
            LoginRequest,
            // ResetSchema,
        },
        jwt::{RefreshToken, ResponseModel as TokenResponse},
    },
    util::auth::{create_jwt, hash_password, verify_jwt_and_get_id, verify_password},
};

const LOGIN_MISSING: &str = "missing email and/or password";
// const LOGIN_RESET: &str = "if account exists with given email, password reset link will be sent";

pub async fn registration_handler(
    State(data): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut conn = data.db_conn.get_conn().await?;

    // Check for email and password
    if body.email().is_empty() || body.password().is_empty() {
        return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, LOGIN_MISSING));
    }

    let schema = body;

    // Check if user exists
    if check_for_email(&conn, schema.email()).await? {
        return Err(ErrorResponse::new(StatusCode::CONFLICT, LOGIN_EXISTS));
    }

    // Hash password
    let password_hash: String = match hash_password(schema.password()) {
        Ok(h) => h,
        Err(e) => return Err(e.into()),
    };

    // Add user
    let schema = LoginInfo::from_request(schema, password_hash);

    register_user(&mut conn, schema).await?;

    Ok(StatusCode::CREATED)
}

pub async fn login_handler(
    State(data): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let conn = data.db_conn.get_conn().await?;

    // Check for email and password
    if body.email().is_empty() || body.password().is_empty() {
        return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, LOGIN_MISSING));
    }

    let schema = body;

    // Check if user exists
    if !check_for_email(&conn, schema.email()).await? {
        return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, LOGIN_FAILED));
    }

    // Get user
    // let login = match get_login_info(&conn, schema.password()).await? {
    //     Some(u) => u,
    //     None => return Err(ErrorResponse::new(StatusCode::NOT_FOUND, LOGIN_FAILED)),
    // };
    let login = get_login_info(&conn, schema.email()).await?;

    // === BELOW IS PROTECTED INFO ===

    // Verify password
    verify_password(login.password_hash(), schema.password())?;

    // Get access JWT
    let access_jwt: String = match create_jwt(&data.encode_key, login.user_id(), None) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };

    let mut response = TokenResponse::new(access_jwt);

    // Get refresh JWT
    // TODO: add an option for people not to get a refresh token somehow
    // For example, in a 'remember me' or 'keep me logged in' option
    match create_jwt(
        &data.encode_key,
        login.user_id(),
        Some(Duration::weeks(1).num_seconds() as u64),
    ) {
        Ok(s) => response.set_refresh_jwt(s),
        Err(e) => return Err(e.into()),
    };

    Ok(Json(json!(response)))
}

pub async fn refresh_handler(
    State(data): State<AppState>,
    Json(body): Json<RefreshToken>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let conn = data.db_conn.get_conn().await?;

    let token = body.refresh_jwt;

    // Verify jwt
    let user_id = verify_jwt_and_get_id(&data.decode_key, &token)?;

    // Check if user exists
    // TODO: what to do in this situation? if a user has a refresh token, they should've had an account
    // Unless they deleted their account
    // Return no account found?
    if !check_for_user(&conn, user_id).await? {
        return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, LOGIN_MISSING));
    }

    // Get access JWT
    let access_jwt: String = match create_jwt(&data.encode_key, user_id, None) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };

    let mut token = TokenResponse::new(access_jwt);

    // Get refresh JWT
    match create_jwt(
        &data.encode_key,
        user_id,
        Some(Duration::weeks(1).num_seconds() as u64),
    ) {
        Ok(s) => token.set_refresh_jwt(s),
        Err(e) => return Err(e.into()),
    };

    Ok(Json(json!(token)))
}

// pub async fn password_reset_handler(
//     State(_data): State<AppState>,
//     Json(_body): Json<ResetSchema>,
// ) -> Result<impl IntoResponse, ErrorResponse> {
//     // TODO: Use data and body above

//     println!("{LOGIN_RESET}");

//     Ok(StatusCode::NOT_IMPLEMENTED)
// }

// TEST: authentication handlers

#[cfg(test)]
mod auth_handler {
    #[test]
    fn todo() {
        assert!(false);
    }
}
