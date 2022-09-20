use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddProductData {
    pub name: String,
    pub description: String,
    pub price: f32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddCartProductData {
    pub product_id: i32,
}
