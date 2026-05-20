use axum::{
    Json, Router,
    routing::{get, post},
};

use crate::controllers::{health_check_controller, limit_order_controller};

pub fn router() -> Router {
    let router: Router = Router::new()
        .route("/health", get(health_check_controller))
        .route("/limit-order/place", post(limit_order_controller()));

    router
}
