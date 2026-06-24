use std::sync::Arc;

use colored::Colorize;
use sea_orm::{ActiveValue::Set, DbErr, EntityTrait, sea_query::OnConflict};
use uuid::Uuid;

use crate::Services;

use super::BOT_USER_ID;

pub async fn load_markets(services: &Arc<Services>) -> Uuid {
    let markets = database::market::Entity::find()
        .all(&services.db)
        .await
        .unwrap_or_default();

    if markets.is_empty() {
        println!("{}", "no markets found, creating BOT/USD".cyan());
        let market_id = create_default_market(services).await;
        services.add_market(market_id, None);
        println!("{}", format!("created market: {}", market_id).cyan());
        return market_id;
    }

    for market in &markets {
        if !services.markets.contains_key(&market.id) {
            services.add_market(market.id, None);
        }
    }

    let market_id = markets[0].id;
    println!(
        "{}",
        format!(
            "{} market(s) loaded, trading on: {}",
            markets.len(),
            market_id
        )
        .cyan()
    );

    market_id
}

async fn create_default_market(services: &Arc<Services>) -> Uuid {
    // ensure bot user exists first (market has a created_by FK)
    let user_model = database::user::ActiveModel {
        id: Set(BOT_USER_ID),
        email: Set("bot@orderbook.internal".to_string()),
        name: Set("Bot".to_string()),
        image: Set(None),
        is_admin: Set(false),
    };

    match database::user::Entity::insert(user_model)
        .on_conflict(
            OnConflict::column(database::user::Column::Email)
                .do_nothing()
                .to_owned(),
        )
        .exec(&services.db)
        .await
    {
        Ok(_) | Err(DbErr::RecordNotInserted) => {}
        Err(e) => eprintln!("failed to create bot user: {e}"),
    }

    let market_model = database::market::ActiveModel {
        id: Set(Uuid::new_v4()),
        currency_a: Set("BOT".to_string()),
        currency_b: Set("USD".to_string()),
        created_by: Set(BOT_USER_ID),
        ..Default::default()
    };

    database::market::Entity::insert(market_model)
        .exec_with_returning(&services.db)
        .await
        .expect("failed to create default market")
        .id
}
