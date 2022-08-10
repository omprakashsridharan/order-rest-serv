use crate::error::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use lib::constants::JWT_SECRET;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn validate_payload<T: Validate>(payload: &T) -> Result<()> {
    Ok(payload.validate()?)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub user_id: i32,
    pub role: String,
}

impl Claims {
    pub fn new(email: String, user_id: i32, role: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: email,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
            user_id,
            role,
        }
    }
}

pub fn sign(email: String, user_id: i32, role: String) -> Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(email, user_id, role),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

pub fn verify(token: &str) -> Result<Claims> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
