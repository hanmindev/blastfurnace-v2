mod visitor;
mod scope_table;

use crate::front::ast_types::Definition;
use crate::front::passes::name_resolution::scope_table::ScopeTable;
use crate::front::passes::visitor::Visitable;
use crate::modules::ModuleId;

#[derive(Debug, PartialEq)]
pub enum NameResolutionError {
    UndefinedVariable(String),
    Redefinition(String),
}

pub struct NameResolver {
    module_id: ModuleId,
    scope_table: ScopeTable,
}
impl NameResolver {
    pub fn run(module_id: ModuleId, definitions: &mut Vec<Definition>) -> Result<(), NameResolutionError> {
        let mut name_resolver = NameResolver {
            module_id,
            scope_table: ScopeTable::new(),
        };

        for definition in definitions.iter_mut() {
            definition.visit(&mut name_resolver)?;
        }

        Ok(())
    }
}