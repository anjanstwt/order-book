use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    Services,
    services::{Err, Response},
    types::AuthUser,
};

#[derive(Deserialize)]
pub struct CancelOrderBody {
    market_id: Uuid,
    order_idx: usize,
}

pub async fn cancel_order_controller(
    Extension(user): Extension<AuthUser>,
    State(service): State<Arc<Services>>,
    Json(body): Json<CancelOrderBody>,
) -> Response<()> {
    let Some(market) = service.markets.get(&body.market_id) else {
        return Response::error(
            StatusCode::NOT_FOUND,
            Some("market not found".to_string()),
            Some(Err::new("NO_MARKET_FOUND".to_string(), None)),
        );
    };

    let mut engine = market.lock().await;

    let Ok(report) = engine.cancel_order(body.order_idx) else {
        return Response::error(
            StatusCode::CONFLICT,
            Some("Failed to cancel your order".to_string()),
            None,
        );
    };

    unimplemented!()
}
