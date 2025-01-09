use std::sync::Arc;
use crate::enums::enums::RegionEnum;
use crate::models::address::Address;
use crate::models::entities::{Item, Warehouse};
use crate::repositories::repositories::WarehouseRepository;

mod models;
mod enums;
mod repositories;

mod errors;
mod data_loader;

fn main() {
   let warehouse_repository = Arc::new(WarehouseRepository::new());

   data_loader::load_data(Arc::clone(&warehouse_repository));
}
