use crate::entities::{item, user, warehouse};
use crate::AppState;
use fake::faker::address::en::{CityName, StateAbbr};
use fake::faker::address::raw::StreetName;
use fake::faker::number::raw::NumberWithFormat;
use fake::locales::EN;
use fake::{faker, rand, Fake, Rng};
use sea_orm::EntityTrait;
use std::error::Error;
use std::sync::Arc;

pub async fn load_data(app_state: &mut AppState) -> Result<(), Box<dyn Error>>{

    let mut warehouse_repository = Arc::clone(&app_state.warehouse_repository.as_mut().unwrap());
    let mut item_repository = Arc::clone(&app_state.item_repository.as_mut().unwrap());
    let mut user_repository = Arc::clone(&app_state.user_repository.as_mut().unwrap());

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

        warehouse_id = warehouse_repository.write().await.create(&warehouse1).await?;

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
            item_repository.write().await.create(&item).await?;
        }


    }


    let user = user::Model {
        id: 0,
        first_name: "Francisco".to_string(),
        last_name: "Juane".to_string(),
        password: "123456".to_string(),
        telephone: "123456789".to_string(),
        email: "admin@admin.com".to_string(),
    };

    user_repository.write().await.create(&user).await;


    Ok(())
}

