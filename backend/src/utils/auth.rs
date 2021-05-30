use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // subject - user id
    pub sub: Uuid,
}

type Token = String;
type ExpiresIn = i64;

pub fn generate_token(user_id: Uuid) -> Result<(Token, ExpiresIn), Box<dyn Error>> {
    let key = env::var("PRIVATE_AUTH_KEY")?;

    let expires_in = Duration::seconds(120);

    let now = Utc::now();
    let expiration = now + expires_in;
    let claims = TokenClaims {
        iat: now.timestamp(),
        exp: expiration.timestamp(),
        sub: user_id,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )?;

    Ok((token, expires_in.num_seconds()))
}

pub fn hash(value: &str) -> bcrypt::BcryptResult<String> {
    bcrypt::hash(value, 12)
}

pub fn verify(password: &str, hash: &str) -> bcrypt::BcryptResult<bool> {
    bcrypt::verify(password, hash)
}
