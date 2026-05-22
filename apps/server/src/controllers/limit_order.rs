use axum::{Extension, Json, extract::State};
use engine::{Quantity, Side, Tick};
use sea_orm::EntityTrait;
use serde::Deserialize;

use crate::{
    services::{Init, Response},
    types::AuthUser,
};

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
    State(state): State<Init>,
) -> Response<()> {
    // check the user balance in db

    let Ok(existing_user) = database::user::Entity::find_by_id(user.id)
        .one(&state.db)
        .await
    else {
        return Response::not_authorized();
    };



    unimplemented!()
}
