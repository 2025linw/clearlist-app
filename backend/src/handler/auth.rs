use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Duration;
use serde_json::json;

use crate::{
    AppState,
    error::Error,
    model::auth::{TokenResponseModel, UserModel},
    schema::auth::{LoginDetails, RefreshToken},
    util::{
        PostgresCmp, SQLQueryBuilder,
        auth::{create_jwt, hash_password, verify_jwt, verify_password},
    },
};

const MISSING_LOGIN: &str = "missing email and/or password";
const ERROR_LOGIN: &str = "user with given email and password combination not found";

pub async fn registration_handler(
    State(data): State<AppState>,
    Json(body): Json<LoginDetails>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check for email and password
    if body.email.is_none() || body.password.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": MISSING_LOGIN,
            })),
        ));
    }

    let email = body.email.unwrap();
    let password = body.password.unwrap();

    // Get database connection
    let mut conn = data.get_conn().await.map_err(|e| e.into())?;

    // Check if user exists
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_condition(UserModel::EMAIL, PostgresCmp::Equal, &email);

    let (statement, params) = query_builder.build_select();

    if conn
        .execute(&statement, &params)
        .await
        .map_err(|e| Error::from(e).into())?
        != 0
    {
        return Err(
            Error::InvalidRequest("user with provided email already exists".to_string()).into(),
        );
    }

    // Hash password
    let password_hash: String = match hash_password(&password) {
        Ok(h) => h,
        Err(e) => return Err(e.into()),
    };

    // Create new user
    let transaction = conn
        .transaction()
        .await
        .map_err(|e| Error::from(e).into())?;

    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_column(UserModel::EMAIL, &email);
    query_builder.add_column(UserModel::PASS_HASH, &password_hash);

    let (statement, params) = query_builder.build_insert();

    transaction
        .execute(&statement, &params)
        .await
        .map_err(|e| Error::from(e).into())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).into())?;

    Ok(StatusCode::CREATED)
}

pub async fn login_handler(
    State(data): State<AppState>,
    Json(body): Json<LoginDetails>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check for email and password
    if body.email.is_none() || body.password.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": MISSING_LOGIN,
            })),
        ));
    }

    let email = body.email.unwrap();
    let password = body.password.unwrap();

    // Get database connection
    let conn = data.get_conn().await.map_err(|e| e.into())?;

    // Check if user exists
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_condition(UserModel::EMAIL, PostgresCmp::Equal, &email);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    let row_opt = conn
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).into())?;

    let user = match row_opt {
        Some(row) => UserModel::from(row),
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": ERROR_LOGIN,
                })),
            ));
        }
    };

    // Verify password
    match verify_password(user.password_hash(), &password) {
        Ok(true) => (),
        Ok(false) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": ERROR_LOGIN,
                })),
            ));
        }
        Err(e) => return Err(e.into()),
    }

    // Get access JWT
    let access_jwt: String = match create_jwt(user.user_id(), None) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };

    let mut response = TokenResponseModel::new(access_jwt);

    // Get refresh JWT
    match create_jwt(
        user.user_id(),
        Some(Duration::weeks(1).num_seconds() as u64),
    ) {
        Ok(s) => response.set_refresh_jwt(s),
        Err(e) => return Err(e.into()),
    };

    Ok(Json(json!(response)))
}

pub async fn refresh_handler(
    State(_data): State<AppState>, // TODO: '_' if completely unused
    Json(body): Json<RefreshToken>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token = body.refresh_jwt;

    match verify_jwt(&token, body.user_id) {
        Ok(true) => (),
        Ok(false) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": "refresh token expired, reauthentication needed",
                })),
            ));
        }
        Err(e) => return Err(e.into()),
    }

    // Get access JWT
    let access_jwt: String = match create_jwt(body.user_id, None) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };

    let mut response = TokenResponseModel::new(access_jwt);

    // Get refresh JWT
    match create_jwt(body.user_id, Some(Duration::weeks(1).num_seconds() as u64)) {
        Ok(s) => response.set_refresh_jwt(s),
        Err(e) => return Err(e.into()),
    };

    Ok(Json(json!(response)))
}

// TODO: Add update user

// TODO: Add delete user

// TEST: authentication handlers
