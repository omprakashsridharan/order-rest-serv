#[cfg(test)]
mod tests {

    use axum::{http::StatusCode, Extension, Json};
    use lib::{clients::get_clients, dto::AddCartProductData, enums::ROLES, utils::jwt::Claims};

    use crate::{
        handler::add_product::handle, repository::cart::MockCartRepository as CartRepository,
    };

    #[tokio::test]
    async fn test_product_already_in_cart() {
        let user_id = 1;
        let product_id = 1;
        let add_cart_product_data = Json(AddCartProductData { product_id });
        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let mut cart_repository_mock = CartRepository::new();
        cart_repository_mock
            .expect_is_product_already_in_cart()
            .returning(|_user_id, _product_id| Ok(true));
        let clients = Extension(get_clients());
        let cart_repository_extension: Extension<CartRepository> = Extension(cart_repository_mock);
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
}
