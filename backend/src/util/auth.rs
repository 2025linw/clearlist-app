use std::{
    fs,
    sync::{Arc, LazyLock},
};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum_jwt_auth::{JwtDecoderState, LocalDecoder};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use uuid::Uuid;

use crate::{error::Error, schema::auth::Claim};

static ARGON2: LazyLock<Argon2> = LazyLock::new(Argon2::default);

static HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(Algorithm::EdDSA));

static VALIDATION: LazyLock<Validation> = LazyLock::new(|| {
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.set_required_spec_claims(&["iss", "aud", "sub", "exp"]);
    validation.set_issuer(&["todo-app-auth"]);
    validation.set_audience(&["todo-app-user"]);

    validation
});

pub static PUBKEY: LazyLock<DecodingKey> = LazyLock::new(|| {
    let key = fs::read("./pubkey.der")
        .map_err(|e| Error::Internal(e.to_string()))
        .expect("unable to read key from file");

    DecodingKey::from_ed_der(&key)
});

pub static PRIVKEY: LazyLock<EncodingKey> = LazyLock::new(|| {
    let key = fs::read("./privkey.der")
        .map_err(|e| Error::Internal(e.to_string()))
        .expect("need private key DER file");

    EncodingKey::from_ed_der(&key)
});

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    Ok(ARGON2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| Error::Internal(e.to_string()))?
        .to_string())
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| Error::Internal(e.to_string()))?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn create_decoder() -> Result<JwtDecoderState<Claim>, Error> {
    let decoder = LocalDecoder::builder()
        .keys(vec![PUBKEY.to_owned()])
        .validation(VALIDATION.to_owned())
        .build()
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok(JwtDecoderState {
        decoder: Arc::new(decoder),
    })
}

pub fn create_jwt(user_id: Uuid, exp: Option<u64>) -> Result<String, Error> {
    let exp = match exp {
        Some(n) => Utc::now().timestamp() as u64 + n,
        None => (Utc::now() + Duration::hours(1)).timestamp() as u64,
    };

    let claims = Claim::new(user_id, exp);

    encode::<Claim>(&HEADER, &claims, &PRIVKEY).map_err(|e| Error::Internal(e.to_string()))
}

pub fn verify_jwt(token: &str, user_id: Uuid) -> Result<bool, Error> {
    let claim = match decode::<Claim>(token, &PUBKEY, &VALIDATION) {
        Ok(t) => t,
        Err(e) => match e.kind() {
            ErrorKind::MissingRequiredClaim(s) => {
                return Err(Error::InvalidRequest(format!("invalid JWT: {}", s)));
            }
            ErrorKind::ExpiredSignature => return Ok(false),
            ErrorKind::InvalidIssuer => {
                return Err(Error::InvalidRequest(
                    "incorrect issuer for JWT".to_string(),
                ));
            }
            ErrorKind::InvalidAudience => {
                return Err(Error::InvalidRequest(
                    "incorrect audience for JWT".to_string(),
                ));
            }
            e => return Err(Error::Internal(format!("{:?}", e))),
        },
    }
    .claims;

    if claim.sub != user_id {
        return Err(Error::InvalidRequest("incorrect user for JWT".to_string()));
    }

    Ok(true)
}
