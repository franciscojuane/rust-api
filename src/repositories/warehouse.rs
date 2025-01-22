use std::ops::Deref;
use std::sync::Arc;
use crate::entities::prelude::Warehouse;
use crate::entities::warehouse;
use crate::errors::errors::CustomError;
use chrono::Utc;
use log::{debug, error, info, trace};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, DeleteResult, EntityTrait, InsertResult, IntoActiveModel, PaginatorTrait, QueryOrder, QuerySelect, TryGetError};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::entities::warehouse::{ActiveModel, Model};

pub struct WarehouseRepository {
    database_connection: Arc<RwLock<DatabaseConnection>>
}

impl WarehouseRepository {
    pub fn new(database_connection: Arc<RwLock<DatabaseConnection>>) -> Self{
        Self {
            database_connection
        }
    }
}

impl WarehouseRepository{
    pub async fn create(&mut self, item: &warehouse::Model) -> Result<i32, CustomError> {
        let warehouse_active_model = warehouse::ActiveModel {
            id: ActiveValue::Set(item.id),
            warehouse_key: ActiveValue::Set(item.warehouse_key.clone()),
            name: ActiveValue::Set(item.name.clone()),
            street: ActiveValue::Set(item.street.clone()),
            number: ActiveValue::Set(item.number),
            city: ActiveValue::Set(item.city.clone()),
            region: ActiveValue::Set(item.region.clone()),
            postal_code: ActiveValue::Set(item.postal_code.clone()),
            creation_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            update_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            effective_time: ActiveValue::Set(item.effective_time),
            expiration_time: ActiveValue::Set(item.expiration_time),
        };
        let db = self.database_connection.write().await;
        let result = Warehouse::insert(warehouse_active_model).exec(&*db).await;
        match result {
            Ok(insert_result) => {
                info!("Warehouse entity created with id {}", &insert_result.last_insert_id);
                Ok(insert_result.last_insert_id)
            }
            Err(error) => {
                error!("Error when creating warehouse: {:?} ", item);
                Err(CustomError::CreationError)
            }
        }
    }


    pub async fn read(&self, id: u64) -> Result<warehouse::Model, CustomError>{
        let db = self.database_connection.read().await;
        let result = Warehouse::find_by_id(id as i32).one(&*db).await;
            match result {
                Ok(item) => {
                    match item{
                        None => {Err(CustomError::ElementNotFound)}
                        Some(warehouse) => {
                            debug!("Read warehouse with id {}", id);
                            Ok(warehouse)

                        }
                    }
                    }
                Err(error) => {
                    match error {
                        DbErr::RecordNotFound(_) => {Err(CustomError::ElementNotFound)},
                        _ => {
                            error!("Error reading warehouse with id {} and error: {} ", id, error.to_string());
                            Err(CustomError::DatabaseError)
                        }
                    }
                    }
            }
    }

    pub async fn update(&mut self, id: i32, warehouse_update_dto: WarehouseUpdateDTO) -> Result<warehouse::Model, CustomError> {
        let result = self.read(id as u64).await;
        let logging_dto = warehouse_update_dto.clone();
        match result {
            Ok(value) => {
                let mut active_model = value.into_active_model();
                if let Some(warehouse_key) = warehouse_update_dto.warehouse_key {
                    active_model.warehouse_key = Set(warehouse_key);
                }
                if let Some(name) = warehouse_update_dto.name {
                    active_model.name = Set(name);
                }
                if let Some(street) = warehouse_update_dto.street {
                    active_model.street = Set(street);
                }
                if let Some(number) = warehouse_update_dto.number {
                    active_model.number = Set(number);
                }
                if let Some(city) = warehouse_update_dto.city {
                    active_model.city = Set(city);
                }
                if let Some(region) = warehouse_update_dto.region {
                    active_model.region = Set(region);
                }
                if let Some(postal_code) = warehouse_update_dto.postal_code {
                    active_model.postal_code = Set(postal_code);
                }
                    active_model.update_time = Set(Some(Utc::now().naive_utc()));
                if let Some(effective_time) = warehouse_update_dto.effective_time {
                    active_model.effective_time = Set(Some(effective_time));
                }
                if let Some(expiration_time) = warehouse_update_dto.expiration_time {
                    active_model.expiration_time = Set(Some(expiration_time));
                }
                let db = self.database_connection.write().await;
                let result = Warehouse::update(active_model).exec(&*db).await;
                match result {
                    Ok(model) => {
                        info!("Warehouse entity updated with id {}", id);
                        Ok(model)
                    }
                    Err(error) => {
                        error!("Error updating warehouse entity with id {} with DTO {:?}", id, logging_dto);
                        Err(CustomError::UpdateError)
                    }
                }
            },
            Err(error) => {
                info!("Error when updating warehouse entity with id {} with values {:?} and error {}", id, logging_dto, error);
                Err(error)
            }
        }
    }


    pub async fn delete(&mut self,  id: u64) -> Result<(), CustomError> {
        let db = self.database_connection.write().await;
        let result = Warehouse::delete_by_id(id as i32).exec(&*db).await;
        match result {
            Ok(delete_result) => {
                if delete_result.rows_affected == 0 {
                    info!("Warehouse entity with id {} not found for update.", id);
                    Result::Err(CustomError::ElementNotFound)
                } else {
                    info!("Warehouse entity with id {} updated.", id);
                    Result::Ok(())
                }
            }
            Err(_) => {
                info!("Warehouse entity with id {} couldn't be deleted", id);
                Err(CustomError::DeletionError)
            }
        }
    }


    pub async fn list(&self, page: u64, page_size:u64) -> Result<Vec<warehouse::Model>, CustomError> {
        let db = self.database_connection.read().await;
        let limit = page_size;
        let offset = limit * (page - 1);
        let results = Warehouse::find()
            .limit(limit)
            .offset(offset)
            .order_by_asc(warehouse::Column::Id)
            .all(&*db)
            .await;

        match results {
            Ok(warehouses) => {Ok(warehouses)},
            Err(E) => {
                error!("Couldn't list warehouses");
                Err(CustomError::ReadError)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WarehouseUpdateDTO {
    pub warehouse_key: Option<String>,
    pub name: Option<String>,
    pub street: Option<String>,
    pub number: Option<i32>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub effective_time: Option<DateTime>,
    pub expiration_time: Option<DateTime>
}



