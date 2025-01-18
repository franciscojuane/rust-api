use crate::entities::warehouse;
use crate::entities::warehouse::Model;
use crate::errors::errors::CustomError;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{delete, get};
use axum::{Json, Router};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub fn warehouse_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list_warehouses).post(create_warehouse))
        .route(
            "/{id}",
            get(get_warehouse)
                .post(update_warehouse)
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

struct ListParams {
    page: Option<u64>,
    page_size: Option<u64>
}
async fn list_warehouses(Query(params) : Query<ListParams>, State(state): State<AppState>) -> impl IntoResponse {
    let result = state.warehouse_repository
        .unwrap()
        .read()
        .await
        .list(params.page, params.page_size)
        .await;

    match  result {
        Ok(warehouses) => {Ok(Json(warehouses))}
        Err(_) => {Err(StatusCode::INTERNAL_SERVER_ERROR)}
    }

}

async fn create_warehouse() -> impl IntoResponse {}

async fn delete_warehouse() -> impl IntoResponse {}

async fn update_warehouse() -> impl IntoResponse {}
