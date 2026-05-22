use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    controllers::health_check_controller, middlewares::auth, routes::order_routes,
    services::Services,
};

pub fn router() -> Router<Arc<Services>> {
    Router::new()
        .route("/health", get(health_check_controller))
        .nest("/order", order_routes())
}
