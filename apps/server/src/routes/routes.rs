use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{controllers::health_check_controller, routes::order_routes, services::Services};

pub fn router() -> Router<Arc<Services>> {
    Router::new()
        .route("/health", get(health_check_controller))
        .nest("/order", order_routes())
}
