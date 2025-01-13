use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use sea_orm::{ActiveValue, DbErr, EntityTrait, InsertResult, IntoActiveModel};
use crate::data_loader::AppState;
use crate::entities::prelude::Warehouse;
use crate::entities::warehouse;
use crate::entities::warehouse::ActiveModel;
use crate::errors::errors::CustomError;


pub struct WarehouseRepository {
    app_state: Arc<AppState>
}

impl WarehouseRepository {
    pub fn new(app_state: Arc<AppState>) -> Self{
        Self {
            app_state
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
            creation_time: ActiveValue::Set(item.creation_time),
            update_time: ActiveValue::Set(item.update_time),
            effective_time: ActiveValue::Set(item.effective_time),
            expiration_time: ActiveValue::Set(item.expiration_time),
        };

        let db = &self.app_state.database_connection;
        Warehouse::insert(warehouse_active_model).exec(db).await.map(|x| x.last_insert_id).map_err(|_| CustomError::CreationError)


    }


    pub async fn read(&self, id: u64) -> Result<warehouse::Model, CustomError>{
        let db = &self.app_state.database_connection;
        Warehouse::find_by_id(id as i32).one(db).await.map(|x| x.unwrap()).map_err(|_| CustomError::ReadError)
    }

    pub async fn update(&mut self, id: i32, item: warehouse::ActiveModel) -> Result<(), CustomError> {
        let db = &self.app_state.database_connection;
        //let warehouse = warehouse::Entity::find_by_id(id as i32).one(db).await.unwrap();
        let mut warehouse_active : warehouse::ActiveModel = item;
        warehouse_active.id = ActiveValue::Set(id);
        Warehouse::update(warehouse_active).exec(db).await.map(|_| ()).map_err(|_| CustomError::UpdateError)
    }

    pub async fn delete(&mut self,  id: u64) -> Result<(), CustomError> {
        let db = &self.app_state.database_connection;
        let delete_result = Warehouse::delete_by_id(id as i32).exec(db).await.unwrap();
        if delete_result.rows_affected == 0 {
            Result::Err(CustomError::ElementNotFound)
        } else {
            Result::Ok(())
        }
    }

}



