use std::error::Error;
use std::sync::Arc;
use crate::enums::enums::RegionEnum;
use crate::models::address::Address;
use crate::repositories::warehouse::WarehouseRepository;
use entities::{prelude::*, *};
mod entities;

mod models;
mod enums;
mod repositories;

mod errors;
mod data_loader;


#[tokio::main]
async fn main() {
   data_loader::load_data().await;
}
