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
use crate::repositories::warehouse::{WarehouseRepository};

pub struct AppState{
    pub database_connection: DatabaseConnection
}

pub async fn load_data() {
    let database_url = "mysql://root:root@localhost:3306/warehouse_db";
    let db = Database::connect(database_url).await.unwrap();

    let appState = Arc::new(AppState{
        database_connection: db
    });

    let mut warehouse_repository = WarehouseRepository::new(appState);

    let warehouse1 = warehouse::Model {
        id: Default::default(),
        warehouse_key: "WAREHOUSE_A".to_string(),
        name: "Warehouse A".to_string(),
        street: "Calle Falsa".to_string(),
        number: 123,
        city: "Salta".to_string(),
        region: "Salta".to_string(),
        postal_code: "4400".to_string(),
        creation_time: Default::default(),
        update_time: None,
        effective_time: None,
        expiration_time: None,
    };

    let new_warehouse = warehouse_repository.create(&warehouse1).await;

    println!("{:?}", &new_warehouse);

    if let Ok(id) = new_warehouse {
        let mut warehouse2 = warehouse1.into_active_model();
        warehouse2.number = ActiveValue::Set(678);
        warehouse2.name = ActiveValue::Set("Warehouse XZ".to_owned());
        warehouse_repository.update((id as i32) - 1, warehouse2).await;
    };

    let x = warehouse_repository.read(7).await;
    println!("{:?}", x.unwrap());
   
}