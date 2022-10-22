use crate::lib::repository::cart::TCartRepository;
use crate::lib::repository::product::TProductRepository;
use crate::lib::{
    dto::AddCartProductData,
    error::{ApiResult, Error},
    utils::jwt::{validate_payload, Claims},
};
use axum::{Extension, Json};
use hyper::StatusCode;
use tracing::{error, info};

pub async fn handle<CR: TCartRepository, PR: TProductRepository>(
    Json(input): Json<AddCartProductData>,
    claims: Claims,
    Extension(cart_repository): Extension<CR>,
    Extension(product_repository): Extension<PR>,
) -> ApiResult<(StatusCode, String)> {
    validate_payload(&input)?;
    info!("Add product to cart request received");
    let product_id = input.product_id;

    let user_id = claims.user_id;

    let product_details = product_repository
        .get_details(product_id)
        .await
        .map_err(|e| {
            error!("Error while getting product details: {e}");
            Error::GetProductDetailsError
        })?;
    info!("{} added to cart", product_details.name);
    cart_repository
        .add_product(user_id, product_id)
        .await
        .map_err(|e| {
            error!("Error while adding product to cart: {e}");
            Error::AddProductToCartError
        })?;
    Ok((
        StatusCode::CREATED,
        String::from("product added successfully"),
    ))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lib::entity::product;
    use crate::lib::repository::cart::MockCartRepository;
    use crate::lib::repository::product::MockProductRepository;
    use crate::lib::{dto::AddCartProductData, enums::ROLES, utils::jwt::Claims};
    use axum::{http::StatusCode, Extension, Json};
    use migration::DbErr;
    use serde_json::Value;

    #[tokio::test]
    async fn test_add_product_already_in_cart() {
        let user_id = 1;
        let product_id = 1;

        let mut cart_repository = MockCartRepository::default();
        cart_repository
            .expect_add_product()
            .return_const(Err(DbErr::Exec(format!(
                "Product id {} already in cart for user id {}",
                product_id, user_id
            ))));

        let mut product_repository = MockProductRepository::default();
        product_repository
            .expect_get_details()
            .return_const(Ok(product::Model {
                id: product_id,
                name: String::from("x"),
                description: "".to_string(),
                price: 0.0,
                created_at: chrono::offset::Local::now().naive_local(),
                updated_at: chrono::offset::Local::now().naive_local(),
            }));

        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let product_repository_extension = Extension(product_repository);
        let cart_repository_extension = Extension(cart_repository);
        let result = handle(
            add_cart_product_data,
            claims,
            cart_repository_extension,
            product_repository_extension,
        )
        .await;
        assert_eq!(result.is_err(), true);
        let err = result.err().unwrap();
        assert_eq!(err.0, StatusCode::INTERNAL_SERVER_ERROR);
        let message = err.1.get("message").unwrap();
        assert_eq!(
            message,
            &Value::String(Error::AddProductToCartError.to_string())
        );
    }

    #[tokio::test]
    async fn test_add_product_get_product_details_error() {
        let user_id = 1;
        let product_id = 1;
        let mut cart_repository = MockCartRepository::default();
        cart_repository.expect_add_product().return_const(Ok(()));

        let mut product_repository = MockProductRepository::default();
        product_repository
            .expect_get_details()
            .return_const(Err(DbErr::Custom("Error".to_owned())));

        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let cart_repository_extension = Extension(cart_repository);
        let product_repository_extension = Extension(product_repository);
        let result = handle(
            add_cart_product_data,
            claims,
            cart_repository_extension,
            product_repository_extension,
        )
        .await;
        assert_eq!(result.is_err(), true);
        let res = result.err().unwrap();
        let message = res.1.get("message").unwrap();
        assert_eq!(res.0, StatusCode::INTERNAL_SERVER_ERROR);
        println!("{}", Error::GetProductDetailsError.to_string());
        assert_eq!(
            message,
            &Value::String(Error::GetProductDetailsError.to_string())
        );
    }

    #[tokio::test]
    async fn test_add_product_successful() {
        let user_id = 1;
        let product_id = 1;
        let mut cart_repository = MockCartRepository::default();
        cart_repository.expect_add_product().return_const(Ok(()));
        let mut product_repository = MockProductRepository::default();
        product_repository
            .expect_get_details()
            .return_const(Ok(product::Model {
                id: product_id,
                name: String::from("x"),
                description: "".to_string(),
                price: 0.0,
                created_at: chrono::offset::Local::now().naive_local(),
                updated_at: chrono::offset::Local::now().naive_local(),
            }));

        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let product_repository_extension = Extension(product_repository);
        let cart_repository_extension = Extension(cart_repository);
        let result = handle(
            add_cart_product_data,
            claims,
            cart_repository_extension,
            product_repository_extension,
        )
        .await;
        assert_eq!(result.is_ok(), true);
        let res = result.ok().unwrap();
        assert_eq!(res.0, StatusCode::CREATED);
    }
}
