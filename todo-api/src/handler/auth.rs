use std::fs;

use argon2::{Argon2, PasswordHash};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng};

use crate::{
    AppState,
    error::Error,
    model::auth::UserModel,
    schema::auth::{Claim, LoginDetails},
    util::{PostgresCmp, SQLQueryBuilder},
};

pub async fn register_user(
    State(data): State<AppState>,
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
    let mut conn = data.get_conn().await.map_err(|e| e.to_axum_response())?;

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
    State(data): State<AppState>,
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
    let conn = data.get_conn().await.map_err(|e| e.to_axum_response())?;

    // Check if user exists
    let mut query_builder = SQLQueryBuilder::new(UserModel::TABLE);
    query_builder.add_condition(UserModel::EMAIL, PostgresCmp::Equal, &email);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    let row_opt = conn
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    let user = match row_opt {
        Some(row) => UserModel::from(row),
        None => {
            return Err(Error::InvalidRequest(
                "Account with email and password combination not found".to_string(),
            )
            .to_axum_response());
        }
    };

    // Verify hash
    let parsed_hash =
        PasswordHash::new(user.password_hash()).map_err(|_| Error::Internal.to_axum_response())?;

    let verify_result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    if let Err(e) = verify_result {
        return match e {
            password_hash::Error::Password => {
                Err(Error::InvalidRequest("Incorrect password".to_string()).to_axum_response())
            }
            _ => Err(Error::Internal.to_axum_response()),
        };
    }

    // Encode key
    let private_key = fs::read("./privkey.der").expect("unable to read key from file"); // TODO: change this to error
    let key = EncodingKey::from_ed_der(&private_key);

    let header = Header::new(Algorithm::EdDSA);

    let exp = Utc::now() + Duration::weeks(1);
    let claims = Claim::new(user.user_id().to_string(), exp.timestamp() as u64);

    let token = encode::<Claim>(&header, &claims, &key).unwrap(); // TODO: change this to error

    Ok(token.into_response())
}

// TODO: Add delete user

// TEST: authentication handlers
