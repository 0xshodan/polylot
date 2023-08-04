use super::super::base::crud::CRUDRepository;
use super::dto::{CreateModuleDto, UpdateModuleDto};
use crate::core::entities::Module;

trait DefinitonRepository: CRUDRepository<Module, CreateModuleDto, UpdateModuleDto> {}
