use std::{sync::Arc, time::Duration};

use axum::{Extension, Json, extract::State, http::StatusCode};
use chrono::Utc;
use engine::{Quantity, Side, Tick};
use events::OrderEvent;
use sea_orm::{EntityTrait, debug_print};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    services::{Producer, Response, Services},
    types::AuthUser,
};

#[derive(Deserialize)]
pub struct LimitOrderBody {
    market_id: Uuid,
    side: Side,
    tick: Tick,
    quantity: Quantity,
}

#[axum::debug_handler]
pub async fn limit_order_controller(
    Extension(user): Extension<AuthUser>,
    State(service): State<Arc<Services>>,
    Json(body): Json<LimitOrderBody>,
) -> Response<()> {
    // check the user balance in db

    let Some(_existing_user) = database::user::Entity::find_by_id(user.id)
        .one(&service.db)
        .await
        .ok()
        .flatten()
    else {
        return Response::not_authorized();
    };

    let order_id = Uuid::new_v4();

    let Some(market) = service.markets.get(&body.market_id) else {
        debug_print!("failed to get the market with id: {} \n", body.market_id);
        return Response::system_error();
    };

    let mut engine = market.lock().await;

    let Ok(report) = engine.submit_limit_order(order_id, body.side, body.tick, body.quantity)
    else {
        return Response::error(
            StatusCode::SERVICE_UNAVAILABLE,
            Some("Failed to place order in the engine".to_string()),
            None,
        );
    };
    drop(engine);

    let event = match OrderEvent::convert(
        body.market_id,
        &report,
        Utc::now(),
        report.resting_order_idx,
    ) {
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
