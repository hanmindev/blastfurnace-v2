use crate::front::ast_types::{ResolvedName, StaticVarDef};
use crate::front::definition_table::DefinitionTable;
use crate::modules::ModuleId;
use std::collections::HashMap;

pub struct GlobalDefinitionTable<'a> {
    pub definition_tables: HashMap<ModuleId, &'a DefinitionTable>,
}

impl<'a> GlobalDefinitionTable<'a> {
    pub fn new() -> GlobalDefinitionTable<'a> {
        GlobalDefinitionTable {
            definition_tables: HashMap::new(),
        }
    }

    pub fn add_definition_table(
        &mut self,
        module_id: ModuleId,
        definition_table: &'a DefinitionTable,
    ) {
        self.definition_tables.insert(module_id, definition_table);
    }

    pub fn get_static_var_definition(&self, name: &ResolvedName) -> Option<&'a StaticVarDef> {
        self.definition_tables
            .get(&name.module_id)
            .and_then(|definition_table| definition_table.static_var_map.get(name))
    }

    pub fn get_var_definition(&self, name: &ResolvedName) -> Option<&'a StaticVarDef> {
        self.definition_tables
            .get(&name.module_id)
            .and_then(|definition_table| definition_table.static_var_map.get(name))
    }

    pub fn get_struct_definition(&self, name: &ResolvedName) -> Option<&'a StaticVarDef> {
        self.definition_tables
            .get(&name.module_id)
            .and_then(|definition_table| definition_table.static_var_map.get(name))
    }

    pub fn get_fn_definition(&self, name: &ResolvedName) -> Option<&'a StaticVarDef> {
        self.definition_tables
            .get(&name.module_id)
            .and_then(|definition_table| definition_table.static_var_map.get(name))
    }
}
