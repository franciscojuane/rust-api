use std::str::Utf8Error;
use chrono::format::Item::Error;
use crate::errors::errors::Errors;
use crate::models::address::Address;
use crate::models::times::Times;



#[derive(Debug)]
pub struct Warehouse {
    pub id: Option<u64>,
    pub warehouse_key: String,
    pub name: String,
    pub address: Address,
    pub items: Vec<Item>,
    pub times: Times
}

impl Warehouse {
    pub fn new(warehouse_key: String, name: String, address: Address) -> Self {
        Self {
            id: Option::None,
            warehouse_key,
            name,
            address,
            items: vec![],
            times: Times::new_with_current_creation_time()
        }
    }

    pub fn add_item(&mut self, item: Item) -> () {
        self.items.push(item);
    }

    pub fn delete_item_by_item_id(&mut self, id: i32) -> Result<Item, Errors> {
        let mut counter = 0;
        for item in self.items.iter() {
            if (item.id.unwrap() == id) {
                break;
            }
            counter += 1;
        }
        if counter == self.items.len() {
            return Err(Errors::ElementNotFound)
        }

        let element_to_remove = self.items.remove(counter);

        Ok(element_to_remove)
    }





}

#[derive(Debug)]
pub struct Item {
    pub id: Option<i32>,
    pub name: String,
    pub units: i32,
    pub price: f32,
    times: Times
}

impl Item {
    pub fn new(name: String, units: i32, price: f32) -> Self {
        Item {
            id: None,
            name,
            units,
            price,
            times: Times::new_with_current_creation_time(),
        }
    }
}




