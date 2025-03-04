//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub units: i32,
    #[sea_orm(column_type = "Double")]
    pub price: f64,
    pub warehouse_id: i32,
    pub creation_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub effective_time: Option<DateTime>,
    pub expiration_time: Option<DateTime>,
    #[sea_orm(column_type = "Double")]
    pub weight: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::warehouse::Entity",
        from = "Column::WarehouseId",
        to = "super::warehouse::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Warehouse,
}

impl Related<super::warehouse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Warehouse.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        Self{
            id: 0,
            name: "".to_string(),
            units: 0,
            price: 0.0,
            warehouse_id: 0,
            creation_time: Some(Utc::now().naive_utc()),
            update_time: Some(Utc::now().naive_utc()),
            effective_time: None,
            expiration_time: None,
            weight: 0.0,
        }
    }
}
