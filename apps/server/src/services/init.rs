use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use dashmap::DashMap;
use database::connect;
use engine::Engine;
use sea_orm::DatabaseConnection;

use crate::types::MarketId;

pub struct Services {
    pub db: DatabaseConnection,
    pub markets: DashMap<MarketId, Mutex<Engine>>,
}

impl Services {
    pub async fn core() -> Arc<Self> {
        let db = connect(&env::var("DATABASE_URL").unwrap()).await;

        Arc::new(Self {
            db: db,
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
