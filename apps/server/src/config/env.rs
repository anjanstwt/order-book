use std::{env, path::PathBuf};

pub struct Env {
    pub database_url: String,
    pub kafka_brokers: String,
    pub server_port: u16,
    pub auth_secret: String,
}

impl Env {
    pub fn load() -> Self {
        let root_env = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.env");

        dotenvy::from_path(root_env).expect(".env not found");

        Self {
            database_url: env::var("DATABASE_URL").expect("database url not found"),
            kafka_brokers: env::var("KAFKA_BROKERS").expect("kafka brokers not found"),
            server_port: env::var("SERVER_PORT")
                .unwrap_or("3000".to_string())
                .parse::<u16>()
                .expect("invalid server port"),
            auth_secret: env::var("AUTH_SECRET").expect("auth secret not found"),
        }
    }
}
