use super::super::super::errors::{
    CreateObjectError, DeleteObjectError, GetObjectError, UpdateObjectError,
};
use super::dto::{CreateDto, UpdateDto};
use async_trait::async_trait;

#[async_trait]
pub trait CRUDRepository<T, C, U>
where
    C: CreateDto,
    U: UpdateDto,
{
    async fn create(&self, dto: C) -> Result<T, CreateObjectError>;
    async fn get_all(&self) -> Result<Vec<T>, GetObjectError>;
    async fn get_by_id(&self, id: u128) -> Result<T, GetObjectError>;
    async fn update(&self, id: u128, dto: U) -> Result<T, UpdateObjectError>;
    async fn remove(&self, id: u128) -> Result<T, DeleteObjectError>;
}
