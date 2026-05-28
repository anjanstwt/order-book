use std::sync::Arc;

use events::OrderEvent;
use rdkafka::{
    Message,
    consumer::{CommitMode, Consumer as KafkaConsumer},
};

use crate::{
    Services,
    services::{DbWriter, Kafka},
};

pub struct Consumer;

impl Consumer {
    pub async fn spawn(services: Arc<Services>) {
        tokio::spawn(async { Consumer::run(services).await });
    }

    async fn run(services: Arc<Services>) {
        let consumer = Kafka::stream_consumer(&services.env.kafka_brokers, "db-writer");

        consumer
            .subscribe(&["orders.events"])
            .expect("failed to subscribe to topics");

        loop {
            let msg = match consumer.recv().await {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("kafka recv error: {e}");
                    return;
                }
            };

            let Some(payload) = msg.payload() else {
                continue;
            };

            let event = match serde_json::from_slice::<OrderEvent>(payload) {
                Ok(event) => event,
                Err(e) => {
                    eprintln!("Invalid event payload: {e}");
                    return;
                }
            };

            if DbWriter::add_order(event, Arc::clone(&services)).await {
                if let Err(e) = consumer.commit_message(&msg, CommitMode::Async) {
                    eprintln!("failed to commit offset: {e}");
                }
            }
        }
    }
}
