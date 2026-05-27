pub use sea_orm_migration::prelude::*;

mod m20260522_031929_create_users;
mod m20260523_165027_update_user_image;
mod m20260525_184648_create_orders;
mod m20260526_120000_alter_order_amounts_to_bigint;
mod m20260527_172931_update_tick_optional;
mod m20260527_173800_create_trade_and_update_order_with_update_time;

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
        ]
    }
}
