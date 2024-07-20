use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::front::ast_types::{Definition, FnDef, Module, ResolvedName, StaticVarDef, StructDef, VarDef};

#[derive(Clone, Serialize, Deserialize)]
pub struct DefinitionTable {
    pub static_var_map: HashMap<ResolvedName, StaticVarDef>,
    pub var_map: HashMap<ResolvedName, VarDef>,
    pub struct_map: HashMap<ResolvedName, StructDef>,
    pub fn_map: HashMap<ResolvedName, FnDef>,
}

impl DefinitionTable {
    pub fn new() -> DefinitionTable {
        DefinitionTable {
            static_var_map: HashMap::new(),
            var_map: HashMap::new(),
            struct_map: HashMap::new(),
            fn_map: HashMap::new(),
        }
    }
}

pub fn collect_definitions(module: &mut Module) -> DefinitionTable {
    let mut definition_table = DefinitionTable::new();
    for definition in module.definitions.as_ref().unwrap() {
        match definition {
            Definition::StaticVarDef(static_var_def) => {
                definition_table.static_var_map.insert(static_var_def.name.resolved.as_ref().unwrap().clone(), static_var_def.clone());
            }
            Definition::VarDef(var_def) => {
                definition_table.var_map.insert(var_def.name.resolved.as_ref().unwrap().clone(), var_def.clone());
            }
            Definition::StructDef(struct_def) => {
                definition_table.struct_map.insert(struct_def.name.resolved.as_ref().unwrap().clone(), struct_def.clone());
            }
            Definition::FnDef(fn_def) => {
                definition_table.fn_map.insert(fn_def.name.resolved.as_ref().unwrap().clone(), fn_def.clone());
            }
        }
    }

    return definition_table;
}