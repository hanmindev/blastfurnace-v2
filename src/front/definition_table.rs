use crate::front::ast_types::{FnDef, ResolvedName, StaticVarDef, StructDef, VarDef};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
