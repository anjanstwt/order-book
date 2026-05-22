use axum::Router;

use crate::routes::router;

pub mod controllers;
pub mod middlewares;
pub mod routes;
pub mod services;
pub mod types;

#[tokio::main]
async fn main() {
    services::Init::env();
    let init = services::Init::core().await;

    let app = Router::new().nest("/", router()).with_state(init);

    let addr = "10.238.187.81:3000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("server started at {addr}");

    axum::serve(listener, app).await.unwrap();
}
