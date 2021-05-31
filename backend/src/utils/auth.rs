use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

lazy_static! {
    pub static ref KEY: String = env::var("PRIVATE_AUTH_KEY").unwrap();
}
const EXPIRE_IN: i64 = 60 * 2;

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // subject - user id
    pub sub: Uuid,
    pub permissions: Vec<String>,
}

impl TokenClaims {
    pub fn from_token(token: &str) -> jsonwebtoken::errors::Result<Self> {
        let token_data = decode::<Self>(
            &token,
            &DecodingKey::from_secret(KEY.as_ref()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}

type Token = String;
type ExpiresIn = i64;

pub fn generate_token(
    user_id: Uuid,
    permissions: Vec<String>,
) -> jsonwebtoken::errors::Result<(Token, ExpiresIn)> {
    let expires_in = Duration::seconds(EXPIRE_IN);

    let now = Utc::now();
    let expiration = now + expires_in;
    let claims = TokenClaims {
        iat: now.timestamp(),
        exp: expiration.timestamp(),
        sub: user_id,
        permissions,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(KEY.as_ref()),
    )?;

    Ok((token, expires_in.num_seconds()))
}

pub fn hash(value: &str) -> bcrypt::BcryptResult<String> {
    bcrypt::hash(value, 12)
}

pub fn verify(password: &str, hash: &str) -> bcrypt::BcryptResult<bool> {
    bcrypt::verify(password, hash)
}
