use axum::{Router, routing::get};

use crate::routes::router;

pub mod controllers;
pub mod middlewares;
pub mod routes;
pub mod services;
pub mod types;

#[tokio::main]
async fn main() {
    services::Init::env();
    services::Init::core();

    let app = Router::new().nest("/", router());

    let addr = "10.238.187.81:3000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("server started at {addr}");

    axum::serve(listener, app).await.unwrap();
}
