use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use sea_orm::{ActiveValue, DbErr, EntityTrait, InsertResult, IntoActiveModel};
use crate::data_loader::AppState;
use crate::entities::prelude::Warehouse;
use crate::entities::{item, warehouse};
use crate::entities::warehouse::ActiveModel;
use crate::errors::errors::CustomError;


pub struct ItemRepository {
    app_state: Arc<AppState>
}

impl ItemRepository {
    pub fn new(app_state: Arc<AppState>) -> Self{
        Self {
            app_state
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
            creation_time: ActiveValue::Set(Some(Utc::now())),
            update_time: ActiveValue::Set(Some(Utc::now())),
            effective_time: Default::default(),
            expiration_time: Default::default(),
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



