use std::error::Error;
use std::fmt::{Display, Formatter};
use sea_orm::DeriveDisplay;

#[derive(Debug)]
pub enum CustomError {
    ElementNotFound,
    CreationError,
    UpdateError,
    ReadError,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom error occurred: {} " , self)
    }
}

impl Error for CustomError {

}

