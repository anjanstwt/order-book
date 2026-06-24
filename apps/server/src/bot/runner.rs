use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use colored::Colorize;
use engine::{Quantity, Side, Status, Tick};
use events::{Metadata, OrderEvent};
use rand::{Rng, SeedableRng, rngs::StdRng};
use uuid::Uuid;

use crate::services::Producer;
use crate::Services;

use super::BOT_USER_ID;

const MID_TICK: Tick = 1000;
const TICK_SPREAD: Tick = 100;

// (market_id, order_idx, side, tick, quantity)
type PendingOrder = (Uuid, usize, Side, Tick, Quantity);

pub async fn run(services: Arc<Services>, market_id: Uuid) {
    let mut rng = StdRng::from_entropy();
    let mut pending: Vec<PendingOrder> = Vec::new();

    println!("{}", "bot started".cyan());

    loop {
        let action = rng.gen_range(0u8..10);

        match action {
            0..=4 => place_limit_order(&services, &mut rng, &mut pending, market_id).await,
            5..=6 => place_market_order(&services, &mut rng, market_id).await,
            _ if !pending.is_empty() => {
                let idx = rng.gen_range(0..pending.len());
                let (m_id, order_idx, side, tick, qty) = pending.swap_remove(idx);
                cancel_order(&services, m_id, order_idx, side, tick, qty).await;
            }
            _ => {}
        }

        tokio::time::sleep(Duration::from_millis(rng.gen_range(200..=1500))).await;
    }
}

async fn place_limit_order(
    services: &Arc<Services>,
    rng: &mut StdRng,
    pending: &mut Vec<PendingOrder>,
    market_id: Uuid,
) {
    let side = if rng.gen_bool(0.5) { Side::Bid } else { Side::Ask };
    let tick: Tick = rng.gen_range((MID_TICK - TICK_SPREAD)..=(MID_TICK + TICK_SPREAD));
    let quantity: Quantity = rng.gen_range(1..=50);
    let order_id = Uuid::new_v4();

    let Some(market) = services.markets.get(&market_id) else {
        return;
    };
    let mut engine = market.lock().await;
    let Ok(report) = engine.submit_limit_order(order_id, side, tick, quantity) else {
        return;
    };
    drop(engine);

    if report.filled_quantity > 0 {
        println!(
            "{}",
            format!(
                "order fulfilled  {} {} x {} (filled {})",
                side_str(side),
                tick,
                quantity,
                report.filled_quantity
            )
            .blue()
        );
    } else {
        println!(
            "{}",
            format!(
                "limit order      {} {} x {}",
                side_str(side),
                tick,
                quantity
            )
            .green()
        );
    }

    if let Some(idx) = report.resting_order_idx {
        pending.push((market_id, idx, side, tick, quantity));
    }

    publish_report(services, market_id, &report, Some(tick)).await;
}

async fn place_market_order(services: &Arc<Services>, rng: &mut StdRng, market_id: Uuid) {
    let side = if rng.gen_bool(0.5) { Side::Bid } else { Side::Ask };
    let quantity: Quantity = rng.gen_range(1..=10);
    let order_id = Uuid::new_v4();

    let Some(market) = services.markets.get(&market_id) else {
        return;
    };
    let mut engine = market.lock().await;
    let Ok(report) = engine.submit_market_order(order_id, side, quantity) else {
        return;
    };
    drop(engine);

    if report.filled_quantity > 0 {
        println!(
            "{}",
            format!(
                "order fulfilled  {} market x {} (filled {})",
                side_str(side),
                quantity,
                report.filled_quantity
            )
            .blue()
        );
    } else {
        println!(
            "{}",
            format!(
                "market order     {} x {} (no fill, empty book)",
                side_str(side),
                quantity
            )
            .magenta()
        );
    }

    publish_report(services, market_id, &report, None).await;
}

async fn cancel_order(
    services: &Arc<Services>,
    market_id: Uuid,
    order_idx: usize,
    side: Side,
    tick: Tick,
    quantity: Quantity,
) {
    let Some(market) = services.markets.get(&market_id) else {
        return;
    };
    let mut engine = market.lock().await;
    let Ok(order_id) = engine.cancel_order(order_idx) else {
        return;
    };
    drop(engine);

    println!(
        "{}",
        format!(
            "order cancelled  {} {} x {}",
            side_str(side),
            tick,
            quantity
        )
        .red()
    );

    let event = OrderEvent {
        metadata: Metadata {
            user_id: BOT_USER_ID,
            market_id,
            timestamp: Utc::now(),
        },
        order_id,
        order_idx: Some(order_idx),
        quantity,
        side,
        tick: Some(tick),
        status: Status::Canceled,
        filled_quantity: 0,
        trades: None,
    };

    publish(services, market_id, event).await;
}

async fn publish_report(
    services: &Arc<Services>,
    market_id: Uuid,
    report: &engine::MatchReport,
    tick: Option<Tick>,
) {
    let Ok(event) = OrderEvent::convert(BOT_USER_ID, market_id, report, Utc::now(), tick) else {
        return;
    };
    publish(services, market_id, event).await;
}

async fn publish(services: &Arc<Services>, market_id: Uuid, event: OrderEvent) {
    let Ok(payload) = serde_json::to_vec(&event) else {
        return;
    };
    if Producer::send(&services.producer, market_id.to_string(), payload)
        .await
        .is_err()
    {
        eprintln!(
            "{}",
            format!("failed to publish event for market {market_id}").red()
        );
    }
}

fn side_str(side: Side) -> &'static str {
    match side {
        Side::Bid => "BID",
        Side::Ask => "ASK",
    }
}
