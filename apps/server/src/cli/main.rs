mod client;
mod prompts;

use colored::Colorize;
use engine::Side;
use uuid::Uuid;

use client::ApiClient;
use prompts::{OrderType, input_quantity, input_text, input_tick, select_order_type, select_side};

#[tokio::main]
async fn main() {
    let email = input_text("Email");
    let name = input_text("Name");
    let market_raw = input_text("Market ID");

    let market_id = match Uuid::parse_str(&market_raw) {
        Ok(id) => id,
        Err(_) => {
            eprintln!("{}", "invalid market UUID".red());
            return;
        }
    };

    let api = match ApiClient::signin(email, name, market_id).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", format!("signin failed: {e}").red());
            return;
        }
    };

    println!("{}", "signed in".green());

    loop {
        println!();
        let Some(order_type) = select_order_type() else {
            println!("{}", "bye".cyan());
            break;
        };

        let side = select_side();

        let result = match order_type {
            OrderType::Limit => {
                let tick = input_tick();
                let quantity = input_quantity();
                let side_label = side_label(side);
                println!(
                    "{}",
                    format!("limit order  {} {} x {}", side_label, tick, quantity).green()
                );
                api.place_limit(side, tick, quantity).await
            }
            OrderType::Market => {
                let quantity = input_quantity();
                let side_label = side_label(side);
                println!(
                    "{}",
                    format!("market order {} x {}", side_label, quantity).magenta()
                );
                api.place_market(side, quantity).await
            }
        };

        match result {
            Ok(msg) => println!("{}", msg.green()),
            Err(e) => eprintln!("{}", format!("error: {e}").red()),
        }
    }
}

fn side_label(side: Side) -> &'static str {
    match side {
        Side::Bid => "BID",
        Side::Ask => "ASK",
    }
}
