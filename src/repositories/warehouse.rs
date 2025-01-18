use std::ops::Deref;
use std::sync::Arc;
use crate::entities::prelude::Warehouse;
use crate::entities::warehouse;
use crate::errors::errors::CustomError;
use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, DeleteResult, EntityTrait, InsertResult, PaginatorTrait, QueryOrder, QuerySelect, TryGetError};
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
                Ok(insert_result.last_insert_id)
            }
            Err(error) => {
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
                        Some(warehouse) => {Ok(warehouse)}
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

    pub async fn update(&mut self, id: i32, item: warehouse::ActiveModel) -> Result<warehouse::Model, CustomError> {
        let mut warehouse_active : warehouse::ActiveModel = item;
        warehouse_active.id = ActiveValue::Set(id);
        let db = self.database_connection.write().await;
        let result = Warehouse::update(warehouse_active).exec(&*db).await;
        match result {
            Ok(value) => {Ok(value)}
            Err(error) => {Err(CustomError::UpdateError)}
        }
    }

    pub async fn delete(&mut self,  id: u64) -> Result<(), CustomError> {
        let db = self.database_connection.write().await;
        let result = Warehouse::delete_by_id(id as i32).exec(&*db).await;
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


    pub async fn list(&self, page: Option<u64>, page_size:Option<u64>) -> Result<Vec<warehouse::Model>, CustomError> {
        let db = self.database_connection.read().await;
        let limit = page_size.unwrap_or_else(|| 30);
        let offset = limit * (page.unwrap_or_else(|| 1) - 1);
        let results = Warehouse::find()
            .limit(limit)
            .offset(offset)
            .order_by_asc(warehouse::Column::Id)
            .all(&*db)
            .await;

        match results {
            Ok(warehouses) => {Ok(warehouses)},
            Err(E) => {Err(CustomError::ReadError)}
        }
    }
}



