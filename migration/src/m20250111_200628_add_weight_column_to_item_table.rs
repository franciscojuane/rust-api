use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager.alter_table(Table::alter().table(Item::Table).add_column( ColumnDef::new(Item::Weight).double().not_null().default(0)).to_owned()).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager.alter_table(Table::alter().table(Item::Table).drop_column(Item::Weight).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Weight
}
