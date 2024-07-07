use crate::front::ast_types::Definition;
use crate::front::passes::name_resolution::NameResolver;
use crate::modules::ModuleId;

mod visitor;
mod name_resolution;
pub fn run_module_pass(module_id: ModuleId, definitions: &mut Vec<Definition>) {
    NameResolver::run(module_id, definitions);
}