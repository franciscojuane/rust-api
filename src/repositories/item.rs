use std::ops::Deref;
use std::sync::Arc;
use crate::entities::prelude::Item;
use crate::entities::item;
use crate::errors::errors::CustomError;
use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
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
        Item::insert(item_active_model).exec(&*db).await.map(|x| x.last_insert_id).map_err(|_| CustomError::CreationError)
     }

    pub async fn read(&self, id: u64) -> Result<item::Model, CustomError>{
        let db = self.database_connection.read().await;
        Item::find_by_id(id as i32).one(&*db).await.map(|x| x.unwrap()).map_err(|_| CustomError::ReadError)
    }

    pub async fn update(&mut self, id: i32, item: item::ActiveModel) -> Result<(), CustomError> {
        let mut item_active: item::ActiveModel = item;
        item_active.id = ActiveValue::Set(id);
        let db = self.database_connection.write().await;
        Item::update(item_active).exec(&*db).await.map(|_| ()).map_err(|_| CustomError::UpdateError)
    }

    pub async fn delete(&mut self,  id: u64) -> Result<(), CustomError> {
        let db = self.database_connection.write().await;
        let delete_result = Item::delete_by_id(id as i32).exec(&*db).await.unwrap();
        if delete_result.rows_affected == 0 {
            Result::Err(CustomError::ElementNotFound)
        } else {
            Result::Ok(())
        }
    }

}



