use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct CreateOrderEvent {
    pub user_id: i32,
    pub order_request_id: String,
}
