pub use sea_orm_migration::prelude::*;

mod m20260522_031929_create_users;
mod m20260523_165027_update_user_image;
mod m20260525_184648_create_orders;
mod m20260526_120000_alter_order_amounts_to_bigint;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260522_031929_create_users::Migration),
            Box::new(m20260523_165027_update_user_image::Migration),
            Box::new(m20260525_184648_create_orders::Migration),
            Box::new(m20260526_120000_alter_order_amounts_to_bigint::Migration),
        ]
    }
}
