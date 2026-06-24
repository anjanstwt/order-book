use dialoguer::{Input, Select, theme::ColorfulTheme};
use engine::Side;

pub enum OrderType {
    Limit,
    Market,
}

pub fn select_order_type() -> Option<OrderType> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Order type")
        .items(&["Limit", "Market", "Exit"])
        .default(0)
        .interact()
        .ok()?;

    match idx {
        0 => Some(OrderType::Limit),
        1 => Some(OrderType::Market),
        _ => None,
    }
}

pub fn select_side() -> Side {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Side")
        .items(&["Bid", "Ask"])
        .default(0)
        .interact()
        .unwrap();

    if idx == 0 { Side::Bid } else { Side::Ask }
}

pub fn input_tick() -> u64 {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Price (tick)")
        .interact_text()
        .unwrap()
}

pub fn input_quantity() -> u64 {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Quantity")
        .interact_text()
        .unwrap()
}

pub fn input_text(prompt: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}
