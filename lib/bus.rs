use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::Bus;
use mockall::automock;
use std::sync::Arc;

pub fn get_bus(rebbitmq_url: String) -> Arc<Bus> {
    let bus = Bus::new_rabbit_bus(rebbitmq_url);
    return Arc::new(bus);
}

#[automock]
pub trait TBus: Send + Sync {
    fn publish_event<T: 'static + BorshDeserialize + BorshSerialize>(
        &self,
        message: T,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl TBus for Bus {
    fn publish_event<T: 'static + BorshDeserialize + BorshSerialize>(
        &self,
        message: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        return self.publish_event(message);
    }
}
