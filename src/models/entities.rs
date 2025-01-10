use std::str::Utf8Error;
use chrono::{DateTime, Utc};
use chrono::format::Item::Error;
use crate::enums::enums::RegionEnum;
use crate::errors::errors::Errors;
use crate::models::address::Address;
use crate::models::times::Times;



#[derive(Debug)]
pub struct Warehouse {
    pub id: Option<u64>,
    pub warehouse_key: String,
    pub name: String,
    pub street: String,
    pub number: i32,
    pub city: String,
    pub region: RegionEnum,
    pub postal_code: String,
    creation_time: Option<DateTime<Utc>>,
    update_time: Option<DateTime<Utc>>,
    effective_time: Option<DateTime<Utc>>,
    expiration_time: Option<DateTime<Utc>>

}

impl Warehouse {
    pub fn new(warehouse_key: String, name: String, street: String, number: i32, city: String, region: RegionEnum, postal_code: String) -> Self {
        Self {
            id: Option::None,
            warehouse_key,
            name,
            street,
            number,
            city,
            region,
            postal_code,
            creation_time: Option::Some(Utc::now()),
            update_time: None,
            effective_time: None,
            expiration_time: None,
        }
    }
}

#[derive(Debug)]
pub struct Item {
    pub id: Option<i32>,
    pub name: String,
    pub units: i32,
    pub price: f32,
    warehouse_id: u64,
    creation_time: Option<DateTime<Utc>>,
    update_time: Option<DateTime<Utc>>,
    effective_time: Option<DateTime<Utc>>,
    expiration_time: Option<DateTime<Utc>>
}

impl Item {
    pub fn new(name: String, units: i32, price: f32, warehouse_id: u64) -> Self {
        Item {
            id: None,
            name,
            units,
            price,
            warehouse_id,
            creation_time: Option::Some(Utc::now()),
            update_time: None,
            effective_time: None,
            expiration_time: None,
        }
    }
}