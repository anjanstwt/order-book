mod market_loader;
mod runner;

pub use market_loader::load_markets;
pub use runner::run;

pub(super) const BOT_USER_ID: uuid::Uuid = uuid::Uuid::nil();
