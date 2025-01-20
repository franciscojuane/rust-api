use crate::entities::item;
use crate::entities::item::Model;
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
use crate::repositories::item::ItemUpdateDTO;

pub fn item_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list_items))
        .route("/", post(create_item))
        .route(
            "/{id}",
            get(get_item)
                .patch(update_item)
                .delete(delete_item),
        )
        .route("/byWarehouse/{id}", get(get_items_by_warehouse))
        .with_state(app_state)
}

async fn get_item(
    Path(id): Path<i32>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = app_state
        .item_repository
        .as_ref()
        .unwrap()
        .read()
        .await
        .read(id as u64)
        .await;
    match result {
        Ok(item) => Ok(Json(item)),
        Err(error) => match error {
            CustomError::ElementNotFound => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

async fn list_items(Query(params) : Query<HashMap<String, u64>>, State(state): State<Arc<AppState>>) -> impl IntoResponse{
    let page =params.get("page").cloned().unwrap_or_else(|| 1);
    let page_size = params.get("size").cloned().unwrap_or_else(|| 10);
    let result = state.item_repository.as_ref()
        .unwrap()
        .read()
        .await
        .list(page, page_size)
        .await;

    match  result {
        Ok(items) => {Ok(Json(items))}
        Err(_) => {Err(StatusCode::INTERNAL_SERVER_ERROR)}
    }

}


async fn create_item(State(state): State<Arc<AppState>>, Json(payload): Json<item::Model>) -> impl IntoResponse {
    let mut result = state.item_repository.as_ref()
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

async fn delete_item(State(state): State<Arc<AppState>>, Path(id): Path<u64>) -> impl IntoResponse {
    let result = state.item_repository.as_ref().unwrap().write().await.delete(id).await;
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

async fn update_item(State(state) : State<Arc<AppState>>,  Path(id) : Path<u64>, Json(item_update_dto): Json<ItemUpdateDTO>) -> impl IntoResponse {
    let result = state.item_repository.as_ref().unwrap().write().await.update(id as i32, item_update_dto).await;
    match result {
        Ok(model) => {
            Ok(Json(model))
        },
        Err(error) => {
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn get_items_by_warehouse(State(state): State<Arc<AppState>>, Path(id) : Path<u64>, Query(params): Query<HashMap<String, u64>>) -> impl IntoResponse {
    let page =params.get("page").cloned().unwrap_or_else(|| 1);
    let page_size = params.get("size").cloned().unwrap_or_else(|| 10);
    let result = state.item_repository.as_ref().unwrap().read().await.list_items_by_warehouse_id(id as i32, page, page_size).await;
    match result {
        Ok(items) => {Ok(Json(items))}
        Err(error) => {
            match error {
                CustomError::ElementNotFound => {Err(StatusCode::NOT_FOUND)}
                CustomError::DatabaseError => {Err(StatusCode::INTERNAL_SERVER_ERROR)},
                _ => {Err(StatusCode::INTERNAL_SERVER_ERROR)}
            }
        }
    }
}
