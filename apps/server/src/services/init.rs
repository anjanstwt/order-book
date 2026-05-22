use std::{env, path::PathBuf, sync::Arc};

use database::connect;
use engine::Engine;
use sea_orm::DatabaseConnection;

pub struct Init {
    pub engine: Engine,
    pub db: DatabaseConnection,
}

impl Init {
    pub async fn core() -> Arc<Self> {
        let db = connect(&env::var("DATABASE_URL").unwrap()).await;

        Arc::new(Self {
            engine: Engine::new(None),
            db: db,
        })
    }

    pub fn env() {
        let root_env = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.env");
        dotenvy::from_path(root_env).expect(".env not setted up");
    }
}
