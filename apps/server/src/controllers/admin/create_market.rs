use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode};
use colored::Colorize;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    Services,
    services::{Err, Response},
    types::AuthUser,
};

#[derive(Deserialize)]
pub struct CreateMarketController {
    currency_a: String,
    currency_b: String,
}

#[derive(Serialize)]
pub struct CreateMarketData {
    market_id: Uuid,
}

pub async fn create_market_controller(
    Extension(user): Extension<AuthUser>,
    State(services): State<Arc<Services>>,
    Json(body): Json<CreateMarketController>,
) -> Response<CreateMarketData> {
    if body.currency_a.eq(&body.currency_b) {
        return Response::error(
            StatusCode::CONFLICT,
            Some("both the curriences are same".to_string()),
            None,
        );
    }

    // this makes the low valued string stick at currency_a
    let (currency_a, currency_b) = if body.currency_a <= body.currency_b {
        (body.currency_a, body.currency_b)
    } else {
        (body.currency_b, body.currency_a)
    };

    let market = database::market::Entity::insert(database::market::ActiveModel {
        currency_a: Set(currency_a.clone()),
        currency_b: Set(currency_b.clone()),
        created_by: Set(user.id),
        ..Default::default()
    })
    .exec_with_returning(&services.db)
    .await;

    let Ok(created_market) = market else {
        return Response::error(
            StatusCode::CONFLICT,
            Some("found market with same currenicies".to_string()),
            Some(Err::new(
                "CONFLICT_OF_SAME_CURRENCIES".to_string(),
                Some(format!(
                    "market of currency {} and currency {} already exists",
                    currency_a, currency_b
                )),
            )),
        );
    };

    println!("{}", format!("[admin] market created   {}/{} ({})", created_market.currency_a, created_market.currency_b, created_market.id).cyan());

    Response::success(
        Some(CreateMarketData {
            market_id: created_market.id,
        }),
        Some("successfully created market".to_string()),
        None,
    )
}
