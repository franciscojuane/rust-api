use crate::entities::warehouse;
use crate::entities::warehouse::Model;
use crate::errors::errors::CustomError;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{delete, get, post};
use axum::{debug_handler, Json, Router};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;
use sea_orm::prelude::DateTime;
use crate::repositories::warehouse::WarehouseUpdateDTO;

pub fn warehouse_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list_warehouses))
        .route("/", post(create_warehouse))
        .route(
            "/{id}",
            get(get_warehouse)
                .patch(update_warehouse)
                .delete(delete_warehouse),
        )
        .with_state(app_state)
}

async fn get_warehouse(
    Path(id): Path<i32>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = app_state
        .warehouse_repository
        .as_ref()
        .unwrap()
        .read()
        .await
        .read(id as u64)
        .await;
    match result {
        Ok(warehouse) => Ok(Json(warehouse)),
        Err(error) => match error {
            CustomError::ElementNotFound => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

async fn list_warehouses(Query(params) : Query<HashMap<String, u64>>, State(state): State<Arc<AppState>>) -> impl IntoResponse{
    let page =params.get("page").cloned().unwrap_or_else(|| 1);
    let page_size = params.get("size").cloned().unwrap_or_else(|| 10);
    let result = state.warehouse_repository.as_ref()
        .unwrap()
        .read()
        .await
        .list(page, page_size)
        .await;

    match  result {
        Ok(warehouses) => {Ok(Json(warehouses))}
        Err(_) => {Err(StatusCode::INTERNAL_SERVER_ERROR)}
    }

}


async fn create_warehouse(State(state): State<Arc<AppState>>, Json(payload): Json<warehouse::Model>) -> impl IntoResponse {
    let mut result = state.warehouse_repository.as_ref()
        .unwrap()
        .write()
        .await
        .create(&payload)
        .await;

    match result {
        Ok(id) => {Ok(Json(id))}
        Err(error) => {Err(StatusCode::BAD_REQUEST)}
    }
}

async fn delete_warehouse(State(state): State<Arc<AppState>>, Path(id): Path<u64>) -> impl IntoResponse {
    let result = state.warehouse_repository.as_ref().unwrap().write().await.delete(id).await;
    match result {
        Ok(_) => {StatusCode::OK}
        Err(error) => {
            match(error){
                CustomError::ElementNotFound => {StatusCode::NOT_FOUND}
                CustomError::DeletionError => {StatusCode::INTERNAL_SERVER_ERROR},
                _ => StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[axum::debug_handler]
async fn update_warehouse(State(state) : State<Arc<AppState>>,  Path(id) : Path<u64>, Json(warehouse_update_dto): Json<WarehouseUpdateDTO>) -> impl IntoResponse {
    let result = state.warehouse_repository.as_ref().unwrap().write().await.update(id as i32, warehouse_update_dto).await;
    match result {
        Ok(model) => {
            Ok(Json(model))
        },
        Err(error) => {
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
