use std::{env, path::PathBuf, sync::Arc};

use dashmap::DashMap;
use database::connect;
use engine::Engine;
use rdkafka::{
    ClientConfig,
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    producer::FutureProducer,
    types::RDKafkaErrorCode,
};
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::types::MarketId;

pub struct Services {
    pub db: DatabaseConnection,
    pub producer: FutureProducer,
    pub markets: DashMap<MarketId, Mutex<Engine>>,
}

impl Services {
    pub async fn core() -> Arc<Self> {
        let db = connect(&env::var("DATABASE_URL").expect("no database url found")).await;

        let kafka_brokers = &env::var("KAFKA_BROKERS").expect("no kafka brokers found");

        // creating the kafka topic
        Services::create_topic("orders.events", kafka_brokers).await;

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

        Arc::new(Self {
            db: db,
            producer,
            markets: DashMap::new(),
        })
    }

    pub fn env() {
        let root_env = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.env");
        dotenvy::from_path(root_env).expect(".env not setted up");
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

    pub fn add_market(&mut self, id: MarketId, capacity_ticks: Option<u64>) {
        self.markets
            .insert(id, Mutex::new(Engine::new(capacity_ticks)));
    }
}
