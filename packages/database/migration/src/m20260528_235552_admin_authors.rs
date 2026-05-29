use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // adding a is_admin col in user table
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        boolean(User::IsAdmin)
                            .not_null()
                            .default(Expr::value(false)),
                    )
                    .to_owned(),
            )
            .await?;

        // adding created by to market
        manager
            .alter_table(
                Table::alter()
                    .table(Market::Table)
                    .add_column(ColumnDef::new(Market::CreatedBy).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-market-creator-user")
                            .from_tbl(Market::Table)
                            .from_col(Market::CreatedBy)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Market::Table)
                    .drop_foreign_key("fk-market-creator-user")
                    .drop_column(Market::CreatedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::IsAdmin)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    IsAdmin,
}

#[derive(DeriveIden)]
enum Market {
    Table,
    CreatedBy,
}
