use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    controllers::{health_check_controller, limit_order_controller},
    middlewares::auth,
};

pub fn router() -> Router {
    let router: Router = Router::new()
        .route("/health", get(health_check_controller))
        .route("/limit-order/place", post(limit_order_controller))
        .layer(middleware::from_fn(auth));

    router
}
