use crate::repositories::item::ItemRepository;
use crate::repositories::warehouse::WarehouseRepository;
use entities::*;
use sea_orm::{Database, DatabaseConnection};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

mod entities;

mod models;
mod enums;
mod repositories;

mod errors;
mod data_loader;
mod api;

pub struct AppState{
   pub database_connection: Option<Arc<RwLock<DatabaseConnection>>>,
   pub warehouse_repository: Option<Arc<RwLock<WarehouseRepository>>>,
   pub item_repository: Option<Arc<RwLock<ItemRepository>>>
}
#[tokio::main]
async fn main() {

   let mut app_state = initialize_app_state().await;
   data_loader::load_data(&mut app_state).await;


}

async fn initialize_app_state() -> AppState {
   let mut app_state = AppState{
      database_connection: Option::None,
      warehouse_repository: Option::None,
      item_repository : Option::None
   };
      let db = initialize_database(&mut app_state).await;
      app_state.database_connection = Some(Arc::new(RwLock::new(db)));
      let x = &app_state.database_connection.as_ref().unwrap();
      let repositories = initialize_repositories(Arc::clone(x));
      app_state.warehouse_repository = Some(Arc::new(RwLock::new(repositories.0)));
      app_state.item_repository = Some(Arc::new(RwLock::new(repositories.1)));

   app_state

}

async fn initialize_database(app_state: &mut AppState) -> DatabaseConnection {
   let database_url = "mysql://root:root@localhost:3306/warehouse_db";
   let db = Database::connect(database_url).await.unwrap();
   db
}

fn initialize_repositories(database_connection: Arc<RwLock<DatabaseConnection>>) -> (WarehouseRepository, ItemRepository) {
   let mut warehouse_repository = WarehouseRepository::new(Arc::clone(&database_connection));
   let mut item_repository = ItemRepository::new(Arc::clone(&database_connection));
   (warehouse_repository, item_repository)
}
