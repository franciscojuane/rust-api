use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use chrono::Utc;
use fake::{faker, rand, Fake, Rng};
use fake::faker::address::en::{CityName, StateAbbr};
use fake::faker::address::raw::StreetName;
use fake::faker::name::en::Name;
use fake::faker::number::raw::NumberWithFormat;
use fake::locales::EN;
use sea_orm::{ActiveValue, Database, DatabaseConnection, EntityTrait, IntoActiveModel};
use crate::entities::prelude::Warehouse;
use crate::entities::prelude::Item;
use crate::entities::{item, warehouse};
use crate::entities::warehouse::Column::{Region, Street};
use crate::enums::enums::RegionEnum;
use crate::models::address::Address;
use crate::repositories::item::ItemRepository;
use crate::repositories::warehouse::{WarehouseRepository};

pub struct AppState{
    pub database_connection: DatabaseConnection
}

pub async fn load_data() -> Result<(), Box<dyn Error>>{
    let database_url = "mysql://root:root@localhost:3306/warehouse_db";
    let db = Database::connect(database_url).await.unwrap();

    let appState = Arc::new(AppState{
        database_connection: db
    });

    let mut warehouse_repository = WarehouseRepository::new(Arc::clone(&appState));
    let mut item_repository = ItemRepository::new(Arc::clone(&appState));

    let mut warehouse_id = 0;
    for i in 1..=10 {
        let name: String = faker::name::en::FirstName().fake();
        let warehouse1 = warehouse::Model {
            id: Default::default(),
            warehouse_key: format!("WAREHOUSE_{}", name.to_uppercase()),
            name: format!("{} Warehouse", name),
            street: StreetName(EN).fake(),
            number: rand::thread_rng().gen_range(1..1000),
            city: CityName().fake(),
            region: StateAbbr().fake(),
            postal_code: NumberWithFormat(EN, "#####").fake(),
            ..Default::default()
        };

        warehouse_id = warehouse_repository.create(&warehouse1).await?;

        let item_list = ["iPhone 14", "iPhone 14 Pro", "iPhone 14 Pro Max", "Samsung Galaxy",
        "Sony Projector", "Asus Display", "Logitech Camera", "Genius Mouse"];
        for j in item_list {
            let item = item::Model{
                id: Default::default(),
                name: j.to_owned(),
                units: rand::thread_rng().gen_range(1..300) ,
                price: ((rand::thread_rng().gen_range(500.00..1000.00) * 100.00) as f64).round() / 100.00,
                warehouse_id,
                ..Default::default()
            };
            item_repository.create(&item).await?;
        }


    }

    Ok(())
}

