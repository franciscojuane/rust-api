use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CustomError {
    ElementNotFound,
    CreationError,
    UpdateError,
    ReadError,
    DatabaseError,
    DeletionError,
    InvalidJwtToken,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom error occurred: {} " , self)
    }
}

impl Error for CustomError {

}

