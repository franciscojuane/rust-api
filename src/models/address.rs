use crate::enums::enums::RegionEnum;
#[derive(Debug)]
pub struct Address{
    pub street: String,
    pub number: i32,
    pub city: String,
    pub region: RegionEnum,
    pub postal_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>
}