use std::ops::Deref;
use std::sync::Arc;
use crate::entities::prelude::{Item, Warehouse};
use crate::entities::item;
use crate::errors::errors::CustomError;
use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use tokio::sync::RwLock;

pub struct ItemRepository {
    database_connection: Arc<RwLock<DatabaseConnection>>
}

impl ItemRepository {
    pub fn new(database_connection: Arc<RwLock<DatabaseConnection>>) -> Self{
        Self {
            database_connection
        }
    }
}

impl ItemRepository{
    pub async fn create(&mut self, item: &item::Model) -> Result<i32, CustomError> {

        let item_active_model = item::ActiveModel {
            id: Default::default(),
            name: ActiveValue::Set(item.name.clone()),
            units: ActiveValue::Set(item.units),
            price: ActiveValue::Set(item.price),
            weight: ActiveValue::Set(item.weight),
            warehouse_id: ActiveValue::Set(item.warehouse_id),
            creation_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            update_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            effective_time: Default::default(),
            expiration_time: Default::default(),
        };
        let db = self.database_connection.write().await;
        let result = Item::insert(item_active_model).exec(&*db).await;
        match result {
            Ok(insert_result) => {
                Ok(insert_result.last_insert_id)
            }
            Err(_) => {
                Err(CustomError::CreationError)
            }
        }}

    pub async fn read(&self, id: u64) -> Result<item::Model, CustomError>{
        let db = self.database_connection.read().await;
        let result = Item::find_by_id(id as i32).one(&*db).await;
        match result {
            Ok(item) => {
                match item{
                    None => {Err(CustomError::ElementNotFound)}
                    Some(element) => {Ok(element)}
                }
            }
            Err(error) => {
                match error {
                    DbErr::RecordNotFound(_) => {Err(CustomError::ElementNotFound)},
                    _ => {Err(CustomError::DatabaseError)}
                }
            }
        }
    }

    pub async fn update(&mut self, id: i32, item: item::ActiveModel) -> Result<item::Model, CustomError> {
        let mut item_active: item::ActiveModel = item;
        item_active.id = ActiveValue::Set(id);
        let db = self.database_connection.write().await;
        let result = Item::update(item_active).exec(&*db).await;
        match result {
            Ok(value) => {Ok(value)}
            Err(error) => {Err(CustomError::UpdateError)}
        }
    }

    pub async fn delete(&mut self,  id: u64) -> Result<(), CustomError> {
        let db = self.database_connection.write().await;
        let result = Item::delete_by_id(id as i32).exec(&*db).await;
        match result {
            Ok(delete_result) => {
                if delete_result.rows_affected == 0 {
                    Result::Err(CustomError::ElementNotFound)
                } else {
                    Result::Ok(())
                }
            }
            Err(_) => {
                Err(CustomError::DeletionError)
            }
        }
    }

}



