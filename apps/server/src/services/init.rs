use std::sync::Arc;

use dashmap::DashMap;
use database::connect;
use engine::Engine;
use rdkafka::producer::FutureProducer;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::{config::Env, services::Kafka, types::MarketId};

pub struct Services {
    pub env: Env,
    pub db: DatabaseConnection,
    pub producer: FutureProducer,
    pub markets: DashMap<MarketId, Mutex<Engine>>,
}

impl Services {
    pub async fn core() -> Arc<Self> {
        // load the environment variables
        let env = Env::load();

        // connect the database
        let db = connect(&env.database_url).await;

        // creating the kafka topic
        Kafka::create_topic("orders.events", &env.kafka_brokers).await;

        // create a new future producer
        let producer = Kafka::future_producer(&env.kafka_brokers);

        // return the thread safe pointer reference
        Arc::new(Self {
            env,
            db: db,
            producer,
            markets: DashMap::new(),
        })
    }

    pub fn add_market(&self, id: MarketId, capacity_ticks: Option<u64>) {
        self.markets
            .insert(id, Mutex::new(Engine::new(capacity_ticks)));
    }
}
