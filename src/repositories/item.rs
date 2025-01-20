use std::ops::Deref;
use std::sync::Arc;
use crate::entities::prelude::{Item, Warehouse};
use crate::entities::{item, warehouse};
use crate::errors::errors::CustomError;
use chrono::Utc;
use log::{debug, error, info};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::entities::item::Model;

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
                info!("Item entity created with id {}", &insert_result.last_insert_id);
                Ok(insert_result.last_insert_id)
            }
            Err(_) => {
                info!("Error when creating item: {:?} ", item);
                Err(CustomError::CreationError)
            }
        }}

    pub async fn read(&self, id: u64) -> Result<item::Model, CustomError>{
        let db = self.database_connection.read().await;
        let result = Item::find_by_id(id as i32).one(&*db).await;
        match result {
            Ok(elem) => {
                match elem{
                    None => {Err(CustomError::ElementNotFound)}
                    Some(item) => {
                        debug!("Read item with id {}", id);
                        Ok(item)

                    }
                }
            }
            Err(error) => {
                match error {
                    DbErr::RecordNotFound(_) => {Err(CustomError::ElementNotFound)},
                    _ => {
                        error!("Error reading item with id {}", id);
                        Err(CustomError::DatabaseError)
                    }
                }
            }
        }
    }

    pub async fn update(&mut self, id: i32, item_update_dto: ItemUpdateDTO) -> Result<item::Model, CustomError> {
        let result = self.read(id as u64).await;
        let logging_dto = item_update_dto.clone();
        match result {
            Ok(value) => {
                let mut active_model = value.into_active_model();
                if let Some(name) = item_update_dto.name {
                    active_model.name = Set(name);
                }
                if let Some(units) = item_update_dto.units {
                    active_model.units = Set(units);
                }
                if let Some(price) = item_update_dto.price {
                    active_model.price = Set(price);
                }
                if let Some(warehouse_id) = item_update_dto.warehouse_id {
                    active_model.warehouse_id = Set(warehouse_id);
                }
                active_model.update_time = Set(Some(Utc::now().naive_utc()));
                if let Some(effective_time) = item_update_dto.effective_time {
                    active_model.effective_time = Set(Some(effective_time));
                }
                if let Some(expiration_time) = item_update_dto.expiration_time {
                    active_model.expiration_time = Set(Some(expiration_time));
                }
                if let Some(weight) = item_update_dto.weight {
                    active_model.weight = Set(weight);
                }

                let db = self.database_connection.write().await;
                let result = Item::update(active_model).exec(&*db).await;
                match result {
                    Ok(model) => {
                        info!("Item entity updated with id {}", id);
                        Ok(model)
                    }
                    Err(error) => {
                        error!("Error updating Item entity with id {} with DTO {:?}", id, logging_dto);
                        Err(CustomError::UpdateError)
                    }
                }
            },
            Err(error) => {
                info!("Error when updating Item entity with id {} with values {:?} and error {}", id, logging_dto, error);
                Err(error)
            }
        }
    }


    pub async fn delete(&mut self,  id: u64) -> Result<(), CustomError> {
        let db = self.database_connection.write().await;
        let result = Item::delete_by_id(id as i32).exec(&*db).await;
        match result {
            Ok(delete_result) => {
                if delete_result.rows_affected == 0 {
                    info!("Item entity with id {} not found for update.", id);
                    Result::Err(CustomError::ElementNotFound)
                } else {
                    info!("Item entity with id {} updated.", id);
                    Result::Ok(())
                }
            }
            Err(_) => {
                info!("Item entity with id {} couldn't be deleted", id);
                Err(CustomError::DeletionError)
            }
        }
    }

    pub async fn list(&self, page: u64, page_size:u64) -> Result<Vec<item::Model>, CustomError> {
        let db = self.database_connection.read().await;
        let limit = page_size;
        let offset = limit * (page - 1);
        let results = Item::find()
            .limit(limit)
            .offset(offset)
            .order_by_asc(item::Column::Id)
            .all(&*db)
            .await;

        match results {
            Ok(items) => {Ok(items)},
            Err(E) => {
                error!("Couldn't list items");
                Err(CustomError::ReadError)
            }
        }
    }

    pub async fn list_items_by_warehouse_id(&self, warehouse_id: i32, page: u64, page_size:u64) -> Result<Vec<item::Model>, CustomError>{
        let db = self.database_connection.read().await;
        let warehouse_result = Warehouse::find_by_id(warehouse_id).one(&*db).await;
        match warehouse_result {
            Ok(model) => {
                let limit = page_size;
                let offset = limit * (page - 1);
                 let item_result = model.unwrap().find_related(Item) .limit(limit)
                     .offset(offset)
                     .order_by_asc(item::Column::Id)
                     .all(&*db)
                     .await;
                match item_result {
                    Ok(items) => {
                        Ok(items)

                    }
                    Err(error) => {
                        error!("Couldn't list items by warehouse");
                        Err(CustomError::DatabaseError)

                    }
                }
            }
            Err(error) => {
                debug!("Warehouse with id {} not found when trying to list items by warehouse", warehouse_id);
                Err(CustomError::ElementNotFound)
            }
        }

    }

}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemUpdateDTO {
    name: Option<String>,
    units: Option<i32>,
    price: Option<f64>,
    warehouse_id: Option<i32>,
    effective_time: Option<DateTime>,
    expiration_time: Option<DateTime>,
    weight: Option<f64>,
}



