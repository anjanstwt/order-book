use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello" }));

    let port = "10.238.187.81:3000";

    let listener = tokio::net::TcpListener::bind(port).await.unwrap();

    println!("your server is started at {port}");

    axum::serve(listener, app).await.unwrap();
}
