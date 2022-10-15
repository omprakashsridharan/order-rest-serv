use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{options::*, BasicProperties, Connection, ConnectionProperties, Result as LapinResult};
use mockall::mock;
use std::error::Error;
use std::sync::Arc;

type PublishResult = Result<(), Box<dyn Error>>;

pub async fn get_bus(rebbitmq_url: String) -> Arc<RabbitBus> {
    let bus = RabbitBus::new(rebbitmq_url)
        .await
        .expect("Unable to establish bus connection");
    return Arc::new(bus);
}

#[axum::async_trait]
pub trait TBus: Clone + Send + Sized + 'static {
    async fn publish_event<T: Clone + Send + Sized + 'static + BorshDeserialize + BorshSerialize>(
        &self,
        message: T,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[axum::async_trait]
impl TBus for RabbitBus {
    async fn publish_event<
        T: Clone + Send + Sized + 'static + BorshDeserialize + BorshSerialize,
    >(
        &self,
        message: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        return self.publish_event(message).await;
    }
}

#[derive(Clone)]
pub struct RabbitBus {
    connection: Arc<Connection>,
}

impl RabbitBus {
    pub async fn new(url: String) -> LapinResult<RabbitBus> {
        let options = ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);
        let connection = Connection::connect(&url, options).await.unwrap();
        Ok(RabbitBus {
            connection: Arc::new(connection),
        })
    }

    // pub async fn subscribe_event<T: 'static>(
    //     &self,
    //     action_name: String,
    //     handler: fn(T) -> (bool, HandleResult),
    // ) -> SubscribeResult
    // where
    //     T: BorshDeserialize,
    // {
    //     let event_name = get_event_name::<T>();

    //     let mut queue_name = event_name.to_owned();
    //     queue_name.push_str(&String::from("."));
    //     queue_name.push_str(&action_name);
    //     let connection = self.connection.clone();
    //     tokio::spawn(async move {
    //         if let Ok(channel) = connection.open_channel(None) {
    //             match channel.queue_declare(
    //                 queue_name.to_owned(),
    //                 QueueDeclareOptions {
    //                     durable: false,
    //                     exclusive: false,
    //                     auto_delete: false,
    //                     ..Default::default()
    //                 },
    //             ) {
    //                 Ok(queue) => {
    //                     let exchange_declare_options = ExchangeDeclareOptions {
    //                         auto_delete: false,
    //                         durable: false,
    //                         internal: false,
    //                         ..Default::default()
    //                     };

    //                     if let Ok(exchange) = &channel.exchange_declare::<String>(
    //                         amiquip::ExchangeType::Fanout,
    //                         event_name.to_owned(),
    //                         exchange_declare_options,
    //                     ) {
    //                         let _ = queue.bind(exchange, "".to_string(), FieldTable::new());

    //                         if let Ok(consumer) = queue.consume(ConsumerOptions::default()) {
    //                             for message in consumer.receiver().iter() {
    //                                 match message {
    //                                     ConsumerMessage::Delivery(delivery) => {
    //                                         let str_message =
    //                                             String::from_utf8_lossy(&delivery.body).to_string();
    //                                         let mut buf = str_message.as_bytes();

    //                                         if let Ok(model) =
    //                                             BorshDeserialize::deserialize(&mut buf)
    //                                         {
    //                                             let handle_result = handler(model);

    //                                             let retry_on_error = handle_result.0;
    //                                             let result = handle_result.1;

    //                                             if result.is_ok() {
    //                                                 let _ = delivery.ack(&channel);
    //                                             } else {
    //                                                 if retry_on_error {
    //                                                     let _ = delivery.nack(&channel, true);
    //                                                 } else {
    //                                                     let _ = delivery.reject(&channel, false);
    //                                                 }
    //                                             }
    //                                         } else {
    //                                             eprintln!("[bus] Error trying to desserialize. Check message format. Message: {:?}", str_message);
    //                                         }
    //                                     }
    //                                     other => {
    //                                         println!("Consumer ended: {:?}", other);
    //                                         break;
    //                                     }
    //                                 }
    //                             }
    //                         } else {
    //                             eprintln!("[bus] Error trying to consume");
    //                         }
    //                     } else {
    //                         eprintln!("[bus] Error declaring exchange");
    //                     }
    //                 }
    //                 Err(err) => eprintln!("[bus] Error creating Queue: {:?}", err),
    //             };
    //         } else {
    //             eprintln!("[bus] Error opening channel");
    //         }
    //     });

    //     Ok(())
    // }

    pub async fn publish_event<T>(&self, message: T) -> PublishResult
    where
        T: BorshDeserialize + BorshSerialize,
    {
        let event_name = get_event_name::<T>();
        let mut buffer = Vec::new();
        message.serialize(&mut buffer)?;
        let channel = self.connection.create_channel().await.unwrap();
        let _publish_result = channel
            .basic_publish(
                &event_name,
                &event_name,
                BasicPublishOptions::default(),
                &buffer,
                BasicProperties::default(),
            )
            .await
            .unwrap()
            .await
            .unwrap();
        Ok(())
    }
}

fn get_event_name<T>() -> String {
    let full_event_name = std::any::type_name::<T>().to_string();
    let event_array = full_event_name.split("::").collect::<Vec<&str>>();
    let event_name = event_array.last().unwrap().to_string();
    event_name
}

mock! {
    pub RabbitBus {}

    impl Clone for RabbitBus {
        fn clone(&self) -> Self;
    }

    #[axum::async_trait]
    impl TBus for RabbitBus {
        async fn publish_event<T: Clone + Send + Sized + 'static + BorshDeserialize + BorshSerialize>(
        &self,
        message: T,
    ) -> Result<(), Box<dyn std::error::Error>>;
    }
}
