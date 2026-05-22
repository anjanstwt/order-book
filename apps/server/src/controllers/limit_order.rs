use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode};
use engine::{Quantity, Side, Tick};
use sea_orm::{EntityTrait, debug_print};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    services::{Response, Services},
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

    let Ok(mut engine) = market.lock() else {
        debug_print!(
            "failed to get the engine of market id: {} \n",
            body.market_id
        );
        return Response::system_error();
    };

    let Ok(report) = engine.submit_limit_order(order_id, body.side, body.tick, body.quantity)
    else {
        return Response::error(
            StatusCode::SERVICE_UNAVAILABLE,
            Some("Failed to place order in the engine".to_string()),
            None,
        );
    };
    drop(engine);

    Response::success(
        Some(()),
        Some("successfully placed order".to_string()),
        None,
    );

    // add the complete report in the database via queue

    unimplemented!()
}
