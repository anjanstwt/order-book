use axum::{Extension, Json};
use engine::{Quantity, Side, Tick};
use serde::Deserialize;

use crate::{services::Response, types::AuthUser};

#[derive(Deserialize)]
struct LimitOrderBody {
    market_id: String,
    side: Side,
    tick: Tick,
    quantity: Quantity,
}

pub async fn limit_order_controller(
    Extension(user): Extension<AuthUser>,
    Json(body): Json<LimitOrderBody>,
) -> Response<()> {
    // check the user balance in db

    unimplemented!()
}
