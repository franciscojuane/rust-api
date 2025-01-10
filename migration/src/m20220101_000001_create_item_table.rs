use sea_orm_migration::{prelude::*, schema::*};
use crate::m20220101_000001_create_warehouse_table::Warehouse;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Item::Id).primary_key().auto_increment().not_null().integer())
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::Units).integer().not_null())
                    .col(ColumnDef::new(Item::Price).double().not_null())
                    .col(ColumnDef::new(Item::WarehouseId).integer().not_null())
                    .col(ColumnDef::new(Item::CreationTime).date_time())
                    .col(ColumnDef::new(Item::UpdateTime).date_time())
                    .col(ColumnDef::new(Item::EffectiveTime).date_time())
                    .col(ColumnDef::new(Item::ExpirationTime).date_time())
                    .foreign_key(ForeignKey::create().name("fk-item-warehouse_id").from(Item::Table, Item::WarehouseId)
                        .to(Warehouse::Table, Warehouse::Id)
                    )

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Item {
    Table,
    Id,
    Name,
    Units,
    Price,
    WarehouseId,
    CreationTime,
    UpdateTime,
    EffectiveTime,
    ExpirationTime
}
