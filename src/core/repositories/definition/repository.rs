use super::super::base::crud::CRUDRepository;
use super::dto::{CreateDefinitionDto, UpdateDefinitionDto};
use crate::core::entities::Definition;

trait DefinitonRepository:
    CRUDRepository<Definition, CreateDefinitionDto, UpdateDefinitionDto>
{
}
