use super::super::base::crud::CRUDRepository;
use super::dto::{CreateCardDto, UpdateCardDto};
use crate::core::entities::Card;

pub trait CardRepository: CRUDRepository<Card, CreateCardDto, UpdateCardDto> {}
