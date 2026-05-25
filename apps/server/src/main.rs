use std::sync::Arc;

use axum::Router;
use uuid::Uuid;

use crate::{routes::router, services::Consumer};

pub mod config;
pub mod controllers;
pub mod middlewares;
pub mod routes;
pub mod services;
pub mod types;

pub use services::Services;

#[tokio::main]
async fn main() {
    // load all the services
    let services = Services::core().await;
    services.add_market(Uuid::nil(), None);
    Consumer::spawn(Arc::clone(&services)).await;

    let app = Router::new().nest("/api/v1", router()).with_state(services);

    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("server started at {addr}");

    axum::serve(listener, app).await.unwrap();
}
