use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    controllers::{health_check_controller, signin_controller},
    routes::{admin_routes, order_routes},
    services::Services,
};

pub fn router() -> Router<Arc<Services>> {
    Router::new()
        .route("/health", get(health_check_controller))
        .route("/signin", post(signin_controller))
        .nest("/order", order_routes())
        .nest("/admin", admin_routes())
}
