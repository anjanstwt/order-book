use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    controllers::{health_check_controller, limit_order_controller},
    middlewares::auth,
    services::Init,
};

pub fn router() -> Router<Arc<Init>> {
    Router::new()
        .route("/health", get(health_check_controller))
        .route("/limit-order/place", post(limit_order_controller))
        .layer(middleware::from_fn(auth))
}
