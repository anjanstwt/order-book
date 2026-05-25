use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode};
use chrono::Utc;
use engine::{Quantity, Side};
use events::OrderEvent;
use sea_orm::{EntityTrait, debug_print};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    Services,
    services::{Producer, Response},
    types::AuthUser,
};

#[derive(Deserialize)]
pub struct MarketOrderBody {
    market_id: Uuid,
    side: Side,
    quantity: Quantity,
}

#[axum::debug_handler]
pub async fn market_order_controller(
    Extension(user): Extension<AuthUser>,
    State(service): State<Arc<Services>>,
    Json(body): Json<MarketOrderBody>,
) -> Response<()> {
    if let Err(_err) = database::user::Entity::find_by_id(user.id)
        .one(&service.db)
        .await
    {
        return Response::not_authorized();
    };

    let Some(market) = service.markets.get(&body.market_id) else {
        debug_print!("failed to get the market with id: {} \n", body.market_id);
        return Response::system_error();
    };

    let mut engine = market.lock().await;

    let order_id = Uuid::new_v4();
    let Ok(report) = engine.submit_market_order(order_id, body.side, body.quantity) else {
        return Response::error(
            StatusCode::SERVICE_UNAVAILABLE,
            Some("Failed to place order in the engine".to_string()),
            None,
        );
    };

    let event = match OrderEvent::convert(body.market_id, &report, Utc::now(), None) {
        Ok(event) => event,
        Err(err) => {
            eprintln!("error while converting report to event {err}");
            return Response::system_error();
        }
    };

    let Ok(payload) = serde_json::to_vec(&event) else {
        eprintln!("Failed to serialize event");
        return Response::system_error();
    };

    match Producer::send(&service.producer, body.market_id.to_string(), payload).await {
        Ok(delivery) => {
            println!(
                "published to partition {}, offset {}",
                delivery.partition, delivery.offset
            );
            Response::success(
                Some(()),
                Some("order placed successfully".to_string()),
                None,
            )
        }
        Err(_e) => Response::error(
            StatusCode::SERVICE_UNAVAILABLE,
            Some("Order went to engine but failed to publish.".to_string()),
            None,
        ),
    }
}
