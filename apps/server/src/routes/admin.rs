use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{Services, controllers::create_market_controller};

pub fn admin_routes() -> Router<Arc<Services>> {
    Router::new().nest("/market", market_routes())
}

fn market_routes() -> Router<Arc<Services>> {
    Router::new().route("/create", post(create_market_controller))
}
