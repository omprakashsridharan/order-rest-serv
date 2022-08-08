use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;

pub struct TokenData {
    pub email: String,
    pub user_id: String,
    pub role: String,
    pub token: Option<String>,
}

pub fn generate_jwt(token_data: TokenData) -> String {
    let secret = env::var("SECRET").expect("SECRET env missing");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", &token_data.email);
    claims.insert("user_id", &token_data.user_id);
    claims.insert("role", &token_data.role);
    let token_str = claims.sign_with_key(&key).unwrap();
    return token_str;
}
