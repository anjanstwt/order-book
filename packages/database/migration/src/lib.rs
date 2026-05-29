pub use sea_orm_migration::prelude::*;

mod m20260522_031929_create_users;
mod m20260523_165027_update_user_image;
mod m20260525_184648_create_orders;
mod m20260526_120000_alter_order_amounts_to_bigint;
mod m20260527_172931_update_tick_optional;
mod m20260527_173800_create_trade_and_update_order_with_update_time;
mod m20260528_193049_remove_optional_remaining_quantity;
mod m20260528_235552_admin_authors;
mod m20260529_014456_currency_checks_for_market;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260522_031929_create_users::Migration),
            Box::new(m20260523_165027_update_user_image::Migration),
            Box::new(m20260525_184648_create_orders::Migration),
            Box::new(m20260526_120000_alter_order_amounts_to_bigint::Migration),
            Box::new(m20260527_172931_update_tick_optional::Migration),
            Box::new(m20260527_173800_create_trade_and_update_order_with_update_time::Migration),
            Box::new(m20260528_193049_remove_optional_remaining_quantity::Migration),
            Box::new(m20260528_235552_admin_authors::Migration),
            Box::new(m20260529_014456_currency_checks_for_market::Migration),
        ]
    }
}
