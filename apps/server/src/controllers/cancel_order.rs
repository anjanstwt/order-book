use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode};
use chrono::Utc;
use engine::Side;
use events::OrderEvent;
use sea_orm::EntityTrait;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    Services,
    services::{Err, Producer, Response},
    types::AuthUser,
};

#[derive(Deserialize)]
pub struct CancelOrderBody {
    market_id: Uuid,
    order_idx: usize,
    side: Side,
}

pub async fn cancel_order_controller(
    Extension(user): Extension<AuthUser>,
    State(service): State<Arc<Services>>,
    Json(body): Json<CancelOrderBody>,
) -> Response<()> {
    if let Err(_err) = database::user::Entity::find_by_id(user.id)
        .one(&service.db)
        .await
    {
        return Response::not_authorized();
    };

    let Some(market) = service.markets.get(&body.market_id) else {
        return Response::error(
            StatusCode::NOT_FOUND,
            Some("market not found".to_string()),
            Some(Err::new("NO_MARKET_FOUND".to_string(), None)),
        );
    };

    let mut engine = market.lock().await;

    let Ok(()) = engine.cancel_order(body.order_idx) else {
        return Response::error(
            StatusCode::CONFLICT,
            Some("Failed to cancel your order".to_string()),
            None,
        );
    };

    let event = OrderEvent::Cancelled {
        market_id: body.market_id,
        timestamp: Utc::now(),
        order_id: Uuid::new_v4(),
        order_idx: Some(body.order_idx),
        side: body.side,
    };

    let Ok(payload) = serde_json::to_vec(&event) else {
        eprintln!("Failed to serialize event");
        return Response::system_error();
    };

    let delivery = Producer::send(&service.producer, body.market_id.to_string(), payload).await;

    match delivery {
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
        Err(_err) => Response::error(
            StatusCode::SERVICE_UNAVAILABLE,
            Some("Order went to engine but failed to publish.".to_string()),
            None,
        ),
    }
}
