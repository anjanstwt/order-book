use std::{env, path::PathBuf, sync::Arc};

use dashmap::DashMap;
use database::connect;
use engine::Engine;
use rdkafka::producer::FutureProducer;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::{services::Kafka, types::MarketId};

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
        Kafka::create_topic("orders.events", kafka_brokers).await;

        let producer = Kafka::future_producer();

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

    pub fn add_market(&mut self, id: MarketId, capacity_ticks: Option<u64>) {
        self.markets
            .insert(id, Mutex::new(Engine::new(capacity_ticks)));
    }
}
