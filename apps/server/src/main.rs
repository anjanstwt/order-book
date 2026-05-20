use axum::{Router, routing::get};

pub mod controllers;
pub mod routes;
pub mod services;

#[tokio::main]
async fn main() {
    services::Init::new();

    let app = Router::new().route("/", get(|| async { "Hello" }));

    let addr = "10.238.187.81:3000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("server started at {addr}");

    axum::serve(listener, app).await.unwrap();
}
