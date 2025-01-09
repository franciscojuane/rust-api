use std::sync::Arc;
use crate::enums::enums::RegionEnum;
use crate::models::address::Address;
use crate::models::entities::{Item, Warehouse};
use crate::repositories::repositories::{Repository, WarehouseRepository};

pub fn load_data(mut arc: Arc<WarehouseRepository>) {
    let mut warehouse1 = Warehouse::new(
        String::from("WAREHOUSE_1"),
        String::from("North Warehouse"),
        Address {
            street: String::from("O'Higgins"),
            number: 160,
            city: String::from("Miami"),
            region: RegionEnum::MA,
            postal_code: String::from("A123A"),
            latitude: None,
            longitude: None,
        }
    );
    println!("{:?}", warehouse1);

    let item1 = Item::new(String::from("Cell Phone"), 23, 45.25);
    let item2 = Item::new(String::from("Microwave"), 4, 23.33);
    let item3 = Item::new(String::from("Smart TV"), 7, 21.95);
    let item4 = Item::new(String::from("BT Speaker"), 3, 12.07);

    warehouse1.add_item(item1);
    warehouse1.add_item(item2);
    warehouse1.add_item(item3);
    warehouse1.add_item(item4);

    println!("{:?}", warehouse1);
}