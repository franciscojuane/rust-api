use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get};
use crate::AppState;

pub fn warehouse_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list_warehouses).post(create_warehouse))
        .route("/:id", get(get_warehouse).post(update_warehouse).delete(delete_warehouse))
}


async fn get_warehouse(){

}

async fn list_warehouses(){

}

async fn create_warehouse(){

}

async fn delete_warehouse() {

}

async fn update_warehouse(){

}