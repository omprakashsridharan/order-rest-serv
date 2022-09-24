use crate::repository::cart::TCartRepository;
use axum::{Extension, Json};
use hyper::StatusCode;
use lib::{
    clients::TClient,
    dto::AddCartProductData,
    error::{ApiResult, Error},
    utils::jwt::{validate_payload, Claims},
};
use tracing::{error, info};

pub async fn handle<C: TClient, CR: TCartRepository>(
    Json(input): Json<AddCartProductData>,
    claims: Claims,
    Extension(cart_repository): Extension<CR>,
    Extension(clients): Extension<C>,
) -> ApiResult<(StatusCode, String)> {
    validate_payload(&input)?;
    info!("Add product to cart request received");
    let product_id = input.product_id;

    let user_id = claims.user_id;

    let product_details = clients
        .get_product_details(product_id)
        .await
        .ok_or(Error::GetProductDetailsError)
        .map_err(|e| {
            error!("Error while getting product details: {e}");
            e
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
    use crate::repository::cart::MockCartRepository;
    use axum::{http::StatusCode, Extension, Json};
    use lib::clients::MockClient;
    use lib::{
        dto::{AddCartProductData, GetProductDetailsResponse},
        enums::ROLES,
        utils::jwt::Claims,
    };
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

        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let mut mock_client = MockClient::new();
        mock_client
            .expect_get_product_details()
            .return_const(Some(GetProductDetailsResponse {
                product_id,
                name: String::from("x"),
            }));
        let clients = Extension(mock_client);
        let cart_repository_extension = Extension(cart_repository);
        let result = handle(
            add_cart_product_data,
            claims,
            cart_repository_extension,
            clients,
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

        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let mut mock_client = MockClient::new();
        mock_client.expect_get_product_details().return_const(None);
        let clients = Extension(mock_client);
        let cart_repository_extension = Extension(cart_repository);
        let result = handle(
            add_cart_product_data,
            claims,
            cart_repository_extension,
            clients,
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

        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let mut mock_client = MockClient::new();
        mock_client
            .expect_get_product_details()
            .return_const(Some(GetProductDetailsResponse {
                product_id,
                name: String::from("x"),
            }));
        let clients = Extension(mock_client);
        let cart_repository_extension = Extension(cart_repository);
        let result = handle(
            add_cart_product_data,
            claims,
            cart_repository_extension,
            clients,
        )
        .await;
        assert_eq!(result.is_ok(), true);
        let res = result.ok().unwrap();
        assert_eq!(res.0, StatusCode::CREATED);
    }
}
