use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetObjectError {
    #[error("invalid id {id} for entity")]
    InvalidId { id: u128 },
}

#[derive(Error, Debug)]
pub enum UpdateObjectError {
    #[error("invalid id {id} for entity")]
    InvalidId { id: u128 },
}
#[derive(Error, Debug)]
pub enum CreateObjectError {
    #[error("invalid id {id} for entity")]
    InvalidId { id: u128 },
}
#[derive(Error, Debug)]
pub enum DeleteObjectError {
    #[error("invalid id {id} for entity")]
    InvalidId { id: u128 },
}
