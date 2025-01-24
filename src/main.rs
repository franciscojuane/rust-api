use crate::repositories::item::ItemRepository;
use crate::repositories::warehouse::WarehouseRepository;
use crate::repositories::user::UserRepository;
use entities::*;
use sea_orm::{Database, DatabaseConnection};
use tower::{buffer::BufferLayer};
use std::error::Error;
use std::sync::Arc;
use axum::middleware::from_fn;
use axum::Router;
use dotenvy::dotenv;
use http::Method;
use log::error;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower::layer::layer_fn;
use tower::ServiceBuilder;
use tower::util::BoxCloneService;
use tower_http::cors::{AllowMethods, AllowOrigin, CorsLayer};
use crate::api::middlewares;
use crate::entities::prelude::User;

mod entities;

mod models;
mod enums;
mod repositories;

mod errors;
mod data_loader;
mod api;
mod auth;

pub struct AppState{
   pub database_connection: Option<Arc<RwLock<DatabaseConnection>>>,
   pub warehouse_repository: Option<Arc<RwLock<WarehouseRepository>>>,
   pub item_repository: Option<Arc<RwLock<ItemRepository>>>,
   pub user_repository: Option<Arc<RwLock<UserRepository>>>
}
#[tokio::main]
async fn main() {

   env_logger::init();

   dotenv().ok();

   let mut app_state = initialize_app_state().await;
   data_loader::load_data(&mut app_state).await;

   let shared_app_state = Arc::new(app_state);

   let auth_routes = auth::auth::get_router(Arc::clone(&shared_app_state));

   let api_routes =
       Router::new()
           .nest("/warehouses", crate::api::warehouse::warehouse_routes(Arc::clone(&shared_app_state)))
           .nest("/items", crate::api::item::item_routes(Arc::clone(&shared_app_state)))
           .layer(from_fn(middlewares::jwt_check_middleware))
       ;

   let all_routes = Router::new()
       .nest("/auth", auth_routes)
       .nest("/api", api_routes)
       .layer(middlewares::get_cors_limit_layer())
       ;

   let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
   println!("->> LISTENING on {:?}\n", listener.local_addr());
   axum::serve(listener, all_routes.into_make_service())
       .await
       .unwrap();


}

async fn initialize_app_state() -> AppState {
   let mut app_state = AppState{
      database_connection: Option::None,
      warehouse_repository: Option::None,
      item_repository : Option::None,
      user_repository: Option::None,
   };
      let db = initialize_database(&mut app_state).await;
      app_state.database_connection = Some(Arc::new(RwLock::new(db)));
      let x = &app_state.database_connection.as_ref().unwrap();
      let repositories = initialize_repositories(Arc::clone(x));
      app_state.warehouse_repository = Some(Arc::new(RwLock::new(repositories.0)));
      app_state.item_repository = Some(Arc::new(RwLock::new(repositories.1)));
      app_state.user_repository = Some(Arc::new(RwLock::new(repositories.2)));

   app_state

}

async fn initialize_database(app_state: &mut AppState) -> DatabaseConnection {
   let database_url = "mysql://root:root@localhost:3306/warehouse_db";
   let db = Database::connect(database_url).await.unwrap();
   db
}

fn initialize_repositories(database_connection: Arc<RwLock<DatabaseConnection>>) -> (WarehouseRepository, ItemRepository, UserRepository) {
   let mut warehouse_repository = WarehouseRepository::new(Arc::clone(&database_connection));
   let mut item_repository = ItemRepository::new(Arc::clone(&database_connection));
   let mut user_repository = UserRepository::new(Arc::clone(&database_connection));
   (warehouse_repository, item_repository, user_repository)
}
