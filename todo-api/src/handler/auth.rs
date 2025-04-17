use std::sync::Arc;

use argon2::{Argon2, PasswordHash};
use axum::{Json, extract::Extension, http::StatusCode, response::IntoResponse};
use password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng};

use crate::{
    AppState,
    error::Error,
    model::auth::UserModel,
    schema::auth::LoginDetails,
    util::{PostgresCmp, SQLQueryBuilder},
};

pub async fn register_user(
    Extension(data): Extension<Arc<AppState>>,
    Json(body): Json<LoginDetails>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check for email and password
    if body.email.is_none() || body.password.is_none() {
        return Err(
            Error::InvalidRequest("Invalid user login details".to_string()).to_axum_response(),
        );
    }

    let email = body.email.unwrap();
    let password = body.password.unwrap();

    // Get database connection
    let mut conn = data
        .get_conn()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Check if user exists
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_condition(UserModel::EMAIL, PostgresCmp::Equal, &email);

    let (statement, params) = query_builder.build_select();

    if conn
        .execute(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?
        != 0
    {
        return Err(Error::InvalidRequest("User already exists".to_string()).to_axum_response());
    }

    // Hash password with Argon2
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::Internal.to_axum_response())?
        .to_string();

    // Create new user
    let transaction = conn
        .transaction()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_column(UserModel::EMAIL, &email);
    query_builder.add_column(UserModel::PASS_HASH, &password_hash);

    let (statement, params) = query_builder.build_insert();

    transaction
        .execute(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    Ok(StatusCode::CREATED)
}

pub async fn login_user(
    Extension(data): Extension<Arc<AppState>>,
    Json(body): Json<LoginDetails>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check for email and password
    if body.email.is_none() || body.password.is_none() {
        return Err(
            Error::InvalidRequest("Invalid user login details".to_string()).to_axum_response(),
        );
    }

    println!("{:#?}", body);

    let email = body.email.unwrap();
    let password = body.password.unwrap();

    // Get database connection
    let conn = data
        .get_conn()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Check if user exists
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_condition(UserModel::EMAIL, PostgresCmp::Equal, &email);
    query_builder.set_return(vec![UserModel::PASS_HASH]);

    let (statement, params) = query_builder.build_select();

    let row_opt = conn
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    let password_hash = match row_opt {
        Some(row) => row.get::<&str, String>(UserModel::PASS_HASH),
        None => {
            return Err(Error::InvalidRequest(
                "Account with email and password combination not found".to_string(),
            )
            .to_axum_response());
        }
    };

    // Verify hash
    let parsed_hash =
        PasswordHash::new(&password_hash).map_err(|_| Error::Internal.to_axum_response())?;

    let verify_result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    match verify_result {
        Ok(()) => Ok(StatusCode::OK),
        Err(password_hash::Error::Password) => {
            Err(Error::InvalidRequest("Incorrect password".to_string()).to_axum_response())
        }
        Err(_) => Err(Error::Internal.to_axum_response()),
    }
}

// Add delete user
