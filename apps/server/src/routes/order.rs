use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    controllers::limit_order::limit_order_controller, middlewares::auth::auth, services::Services,
};

pub fn order_routes() -> Router<Arc<Services>> {
    Router::new()
        .nest("/limit", limit_routes())
        .next("/market", market_routes())
        .layer(middleware::from_fn(auth))
}

fn limit_routes() -> Router<Arc<Services>> {
    Router::new().route("/place", post(limit_order_controller))
}

fn market_routes() -> Router<Arc<Services>> {
    Router::new().route("/place", get(|| "hello"))
}
