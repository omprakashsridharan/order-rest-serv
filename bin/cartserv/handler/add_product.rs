use crate::repository::cart::CartRepository;
use axum::{Extension, Json};
use hyper::StatusCode;
use lib::{
    clients::TClient,
    dto::AddCartProductData,
    error::{ApiError, ApiResult, Error},
    utils::jwt::{validate_payload, Claims},
};
use tracing::{error, info};

pub async fn handle<T: TClient>(
    Json(input): Json<AddCartProductData>,
    claims: Claims,
    Extension(cart_repository): Extension<CartRepository>,
    Extension(clients): Extension<T>,
) -> ApiResult<(StatusCode, String)> {
    validate_payload(&input)?;
    info!("Add product to cart request received");
    let product_id = input.product_id;

    let user_id = claims.user_id;
    let is_product_already_in_cart = cart_repository
        .is_product_already_in_cart(user_id, product_id)
        .await
        .map_err(|e| {
            error!("Error while checking if product is in cart: {e}");
            Error::AddProductToCartError
        })?;

    if !is_product_already_in_cart {
        let product_details = clients
            .get_product_details(product_id)
            .await
            .ok_or(Error::GetProductDetailsError)
            .unwrap();
        info!("{} added to cart", product_details.name);
        cart_repository
            .add_product(user_id, product_id)
            .await
            .map_err(|e| {
                error!("Error while adding product to cart: {e}");
                Error::AddProductToCartError
            })
            .unwrap();
        Ok((
            StatusCode::CREATED,
            String::from("product added successfully"),
        ))
    } else {
        error!("Product already in cart");
        Err(ApiError::from(Error::ProductAlreadyInCartError))
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use crate::entity::cart;

    use super::*;
    use axum::{http::StatusCode, Extension, Json};
    use lib::{
        dto::{AddCartProductData, GetProductDetailsResponse},
        enums::ROLES,
        utils::jwt::Claims,
    };
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    #[derive(Clone)]
    struct MockClient {}

    #[axum::async_trait]
    impl TClient for MockClient {
        async fn get_product_details(&self, product_id: i32) -> Option<GetProductDetailsResponse> {
            Some(GetProductDetailsResponse {
                product_id,
                name: String::from("x"),
            })
        }
    }

    #[tokio::test]
    async fn test_product_already_in_cart() {
        let user_id = 1;
        let product_id = 1;
        let db_pool = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![cart::Model {
                user_id,
                product_id,
                ..Default::default()
            }]])
            .into_connection();
        let cart_repository = CartRepository {
            db_pool: Arc::new(db_pool),
        };

        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let clients = Extension(MockClient {});
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
        assert_eq!(message, "Product is already in cart");
    }

    #[tokio::test]
    async fn test_product_not_in_cart() {
        let user_id = 1;
        let product_id = 1;
        let db_pool = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results::<cart::Model>(vec![vec![]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let cart_repository = CartRepository {
            db_pool: Arc::new(db_pool),
        };

        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let clients = Extension(MockClient {});
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
        let message = res.1;
        assert_eq!(res.0, StatusCode::CREATED);
        assert_eq!(message, "product added successfully");
    }
}
