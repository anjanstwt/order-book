use std::time::Duration;

use rdkafka::{
    producer::{FutureProducer, FutureRecord, future_producer::Delivery},
    util::Timeout,
};

pub struct Producer;

impl Producer {
    pub async fn send(
        producer: &FutureProducer,
        key: String,
        payload: Vec<u8>,
    ) -> Result<Delivery, ()> {
        let record = FutureRecord::to("order.events").key(&key).payload(&payload);

        let delivery = producer
            .send(record, Timeout::After(Duration::from_secs(5)))
            .await
            .map_err(|(e, _)| eprintln!("failed to publish order event {e}"))?;

        Ok(delivery)
    }
}
