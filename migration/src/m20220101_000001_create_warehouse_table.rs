use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Warehouse::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Warehouse::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Warehouse::WarehouseKey).string().not_null())
                    .col(ColumnDef::new(Warehouse::Name).string().not_null())
                    .col(ColumnDef::new(Warehouse::Street).string().not_null())
                    .col(ColumnDef::new(Warehouse::Number).integer().not_null())
                    .col(ColumnDef::new(Warehouse::City).string().not_null())
                    .col(ColumnDef::new(Warehouse::Region).string().not_null())
                    .col(ColumnDef::new(Warehouse::PostalCode).string().not_null())
                    .col(ColumnDef::new(Warehouse::CreationTime).date_time())
                    .col(ColumnDef::new(Warehouse::UpdateTime).date_time())
                    .col(ColumnDef::new(Warehouse::EffectiveTime).date_time())
                    .col(ColumnDef::new(Warehouse::ExpirationTime).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //todo!();

        manager
            .drop_table(Table::drop().table(Warehouse::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Warehouse {
    Table,
    Id,
    WarehouseKey,
    Name,
    Street,
    Number,
    City,
    Region,
    PostalCode,
    CreationTime,
    UpdateTime,
    EffectiveTime,
    ExpirationTime
}
