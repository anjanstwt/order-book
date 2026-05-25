use std::env;

use rdkafka::{
    ClientConfig,
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    producer::FutureProducer,
    types::RDKafkaErrorCode,
};

pub struct Kafka;

impl Kafka {
    pub fn future_producer() -> FutureProducer {
        let kafka_brokers = &env::var("KAFKA_BROKERS").expect("no kafka brokers found");

        let producer = ClientConfig::new()
            .set("bootstrap.servers", kafka_brokers)
            .set("client.id", "order-book-id")
            .set("acks", "all")
            .set("enable.idempotence", "true")
            .set("compression.type", "lz4")
            .set("linger.ms", "5")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Failed to create producer");

        producer
    }

    pub async fn create_topic(topic: &str, kafka_brokers: &str) {
        let admin_client = ClientConfig::new()
            .set("bootstrap.servers", kafka_brokers)
            .create::<AdminClient<_>>()
            .expect("Failed to create admin client");

        let topic = NewTopic::new(topic, 6, TopicReplication::Fixed(1));

        let options = AdminOptions::new();
        let results = admin_client
            .create_topics([&topic], &options)
            .await
            .expect("Topic creation failed");

        for result in results {
            match result {
                Ok(t) => println!("Topic: {t} created"),
                Err((name, err)) => match err {
                    RDKafkaErrorCode::TopicAlreadyExists => {
                        println!("Topic {name} is already there")
                    }
                    _ => println!("Error creating {}: {}", name, err),
                },
            }
        }
    }
}
