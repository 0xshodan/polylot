use super::super::super::errors::{
    CreateObjectError, DeleteObjectError, GetObjectError, UpdateObjectError,
};
use crate::core::entities::Card;
use crate::core::repositories::card::CardRepository;

pub struct CardService<T: CardRepository> {
    pub card_repository: T,
}

impl<T: CardRepository> CardService<T> {
    async fn get_all(&self) -> Result<Vec<Card>, GetObjectError> {
        self.card_repository.get_all().await
    }
}
