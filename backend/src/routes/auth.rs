use axum::{Json, body, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Duration;
use serde_json::json;

use crate::{
    AppState,
    error::{ErrorResponse, LOGIN_AUTH, LOGIN_EXISTS},
    models::{
        ToResponse,
        auth::{
            LoginSchema, ResetSchema, UserTokenResponseModel,
            token::{RefreshToken, ResponseModel as TokenResponse},
            user::DatabaseModel as UserModel,
        },
    },
    util::{
        PostgresCmp, SQLQueryBuilder,
        auth::{create_jwt, hash_password, verify_jwt, verify_password},
    },
};

const LOGIN_MISSING: &str = "missing email and/or password";
const LOGIN_RESET: &str = "if account exists with given email, password reset link will be sent";

pub async fn registration_handler(
    State(data): State<AppState>,
    Json(body): Json<LoginSchema>,
) -> Result<impl IntoResponse, ErrorResponse> {
    // Check for email and password
    if body.email.is_empty() || body.password.is_empty() {
        return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, LOGIN_MISSING));
    }

    let LoginSchema { email, password } = body;

    // Check if user exists
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_condition(UserModel::EMAIL, PostgresCmp::Equal, &email);

    let (statement, params) = query_builder.build_select();

    if data
        .db_conn
        .query_select_one(statement, params)
        .await?
        .is_some()
    {
        return Err(ErrorResponse::new(StatusCode::CONFLICT, LOGIN_EXISTS));
    }

    // Hash password
    let password_hash: String = match hash_password(&password) {
        Ok(h) => h,
        Err(e) => return Err(e.into()),
    };

    // Create new user
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_column(UserModel::EMAIL, &email);
    query_builder.add_column(UserModel::PASS_HASH, &password_hash);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_insert();

    data.db_conn.query_insert(statement, params).await?;

    Ok(StatusCode::CREATED)
}

pub async fn login_handler(
    State(data): State<AppState>,
    Json(body): Json<LoginSchema>,
) -> Result<impl IntoResponse, ErrorResponse> {
    // Check for email and password
    if body.email.is_empty() || body.password.is_empty() {
        return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, LOGIN_MISSING));
    }

    let LoginSchema { email, password } = body;

    // Check if user exists
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_condition(UserModel::EMAIL, PostgresCmp::Equal, &email);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    let row = match data.db_conn.query_select_one(statement, params).await? {
        Some(r) => r,
        None => return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, LOGIN_AUTH)),
    };

    let user = UserModel::from(row);

    // Verify password
    verify_password(user.password_hash(), &password)?;

    // Get access JWT
    let access_jwt: String = match create_jwt(&data.encode_key, user.user_id(), None) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };

    let mut response = TokenResponse::new(access_jwt);

    // Get refresh JWT
    match create_jwt(
        &data.encode_key,
        user.user_id(),
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
    let token = body.refresh_jwt;

    // Verify jwt
    verify_jwt(&data.decode_key, &token, body.user_id)?;

    // Check if user exists
    // TODO: what to do in this situation? if a user has a refresh token, they should've had an account
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_condition(UserModel::ID, PostgresCmp::Equal, &body.user_id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    let row = match data.db_conn.query_select_one(statement, params).await? {
        Some(r) => r,
        None => return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, LOGIN_AUTH)),
    };

    let user = UserModel::from(row);

    // Get access JWT
    let access_jwt: String = match create_jwt(&data.encode_key, body.user_id, None) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };

    let mut token = TokenResponse::new(access_jwt);

    // Get refresh JWT
    match create_jwt(
        &data.encode_key,
        body.user_id,
        Some(Duration::weeks(1).num_seconds() as u64),
    ) {
        Ok(s) => token.set_refresh_jwt(s),
        Err(e) => return Err(e.into()),
    };

    let response = UserTokenResponseModel::new(user.to_response(), token);

    Ok(Json(json!(response)))
}

pub async fn password_reset_handler(
    State(data): State<AppState>,
    Json(body): Json<ResetSchema>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}

// TODO: Add update user

// TODO: Add delete user
// Make deleting be a flag rather than removal of account, just in case that user wants to reopen account
// have delete occur after 1 month
// utilize a `modified_date` to test when an accound was deleted

// TODO: Add password reset mechanic

// TEST: authentication handlers
