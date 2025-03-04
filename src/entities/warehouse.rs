//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "warehouse")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub warehouse_key: String,
    pub name: String,
    pub street: String,
    pub number: i32,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub creation_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub effective_time: Option<DateTime>,
    pub expiration_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::item::Entity")]
    Item,
}

impl Related<super::item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: 0,
            warehouse_key: "".to_string(),
            name: "".to_string(),
            street: "".to_string(),
            number: 0,
            city: "".to_string(),
            region: "".to_string(),
            postal_code: "".to_string(),
            creation_time: Some(Utc::now().naive_utc()),
            update_time: Some(Utc::now().naive_utc()),
            effective_time: None,
            expiration_time: None,
        }
    }
}