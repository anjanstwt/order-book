use axum::Router;

use crate::routes::router;

pub mod controllers;
pub mod middlewares;
pub mod routes;
pub mod services;
pub mod types;

pub use services::Services;

#[tokio::main]
async fn main() {
    Services::env();
    let services = Services::core().await;

    let app = Router::new().nest("/api/v1", router()).with_state(services);

    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("server started at {addr}");

    axum::serve(listener, app).await.unwrap();
}
