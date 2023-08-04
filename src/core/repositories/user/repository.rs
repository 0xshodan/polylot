use super::super::base::crud::CRUDRepository;
use super::dto::{CreateUserDto, UpdateUserDto};
use crate::core::entities::User;

trait DefinitonRepository: CRUDRepository<User, CreateUserDto, UpdateUserDto> {}
