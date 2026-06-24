use std::sync::Arc;

use axum::{Extension, Router};
use crate::{routes::router, services::Consumer};

pub mod bot;
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
    let bot_market_id = bot::load_markets(&services).await;
    Consumer::spawn(Arc::clone(&services)).await;
    tokio::spawn(bot::run(Arc::clone(&services), bot_market_id));

    let app = Router::new()
        .nest("/api/v1", router())
        // the extension layer here is to pass services in middlewares
        .layer(Extension(Arc::clone(&services)))
        .with_state(services);

    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("server started at {addr}");

    axum::serve(listener, app).await.unwrap();
}
