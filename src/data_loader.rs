use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use chrono::Utc;
use sea_orm::{ActiveValue, Database, DatabaseConnection, EntityTrait, IntoActiveModel};
use crate::entities::prelude::Warehouse;
use crate::entities::prelude::Item;
use crate::entities::{item, warehouse};
use crate::entities::warehouse::Column::Region;
use crate::enums::enums::RegionEnum;
use crate::models::address::Address;
use crate::repositories::item::ItemRepository;
use crate::repositories::warehouse::{WarehouseRepository};

pub struct AppState{
    pub database_connection: DatabaseConnection
}

pub async fn load_data() -> Result<(), dyn Error>{
    let database_url = "mysql://root:root@localhost:3306/warehouse_db";
    let db = Database::connect(database_url).await.unwrap();

    let appState = Arc::new(AppState{
        database_connection: db
    });

    let mut warehouse_repository = WarehouseRepository::new(Arc::clone(&appState));
    let mut item_repository = ItemRepository::new(Arc::clone(&appState));


    let warehouse1 = warehouse::Model {
        id: Default::default(),
        warehouse_key: "MAIN_WAREHOUSE".to_string(),
        name: "Main Warehouse".to_string(),
        street: "Fake Street".to_string(),
        number: 123,
        city: "Miami".to_string(),
        region: "FL".to_string(),
        postal_code: "12345".to_string(),
        ..Default::default()
    };

    let warehouse1_id = warehouse_repository.create(&warehouse1).await?;

    let item1 = item::Model{
        id: Default::default(),
        name: "iPhone 14 ".to_owned(),
        units: 23,
        price: 500.00,
        warehouse_id: warehouse1_id,
        ..Default::default()
    };

    let item1_id = item_repository.create(&item1).await?;

    let item2 = item::Model{
        id: Default::default(),
        name: "iPhone 14 Pro ".to_owned(),
        units: 38,
        price: 700.00,
        warehouse_id: warehouse1_id,
        ..Default::default()
    };

    let item2_id = item_repository.create(&item2).await?;

    let item3 = item::Model{
        id: Default::default(),
        name: "iPhone 14 Pro Max".to_owned(),
        units: 15,
        price: 900.00,
        warehouse_id: warehouse1_id,
        ..Default::default()
    };

    let item3_id = item_repository.create(&item3).await?;

    Ok(())
}

