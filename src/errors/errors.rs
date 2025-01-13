#[derive(Debug)]
pub enum CustomError {
    ElementNotFound,
    CreationError,
    UpdateError,
    ReadError,
}