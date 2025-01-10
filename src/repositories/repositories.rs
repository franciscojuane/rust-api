use std::error::Error;
use crate::errors::errors::Errors;
use crate::models::entities::{Item, Warehouse};

pub trait Repository<T> {
    fn create(&mut self, item: T);
    fn find_by_id(&self,id: u64) -> Option<&T>;
    fn update(&mut self, id: u64, item: T);
    fn delete(&mut self, id: u64) -> Result<T, Errors>;
    fn list(&self) -> Vec<&T>;
}

pub struct WarehouseRepository {
    items: Vec<Warehouse>
}

impl WarehouseRepository {
    pub fn new() -> Self {
        WarehouseRepository {
            items: vec![]
        }
    }
}

impl Repository<Warehouse> for WarehouseRepository {
    fn create(&mut self, item: Warehouse) {
        self.items.push(item);
    }

    fn find_by_id(&self, id: u64) -> Option<&Warehouse> {
        for item in self.items.iter() {
            if (item.id.unwrap() == id){
                return Option::Some(item);
            }
        }
        Option::None
    }

    fn update(&mut self, id: u64, mut item: Warehouse) {
        self.delete(id);
        item.id = Option::Some(id);
        self.create(item);
    }

    fn delete(&mut self, id: u64) -> Result<Warehouse, Errors> {
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

    fn list(&self) -> Vec<&Warehouse> {
        self.items.iter().collect()
    }
}

pub struct ItemRepository {
    items: Vec<Item>
}

impl ItemRepository {
    pub fn new() -> Self {
        ItemRepository {
            items: vec![]
        }
    }
}




