use crate::front::ast_types::Definition;
use crate::front::passes::name_resolution::NameResolver;
use crate::modules::ModuleId;

pub mod name_resolution;
mod visitor;