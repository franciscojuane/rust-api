pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_warehouse_table;
mod m20220101_000001_create_item_table;
mod m20250111_200628_add_weight_column_to_item_table;
mod m20250123_153348_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_warehouse_table::Migration),
            Box::new(m20220101_000001_create_item_table::Migration),
            Box::new(m20250111_200628_add_weight_column_to_item_table::Migration),
            Box::new(m20250123_153348_create_user_table::Migration),
        ]
    }
}
