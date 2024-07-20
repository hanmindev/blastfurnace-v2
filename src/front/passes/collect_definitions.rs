use crate::front::ast_types::{Definition, Module};
use crate::front::definition_table::DefinitionTable;

pub fn collect_definitions(module: &mut Module) -> DefinitionTable {
    let mut definition_table = DefinitionTable::new();
    for definition in module.definitions.take().unwrap() {
        match definition {
            Definition::StaticVarDef(static_var_def) => {
                definition_table.static_var_map.insert(
                    static_var_def.name.resolved.as_ref().unwrap().clone(),
                    static_var_def,
                );
            }
            Definition::VarDef(var_def) => {
                definition_table
                    .var_map
                    .insert(var_def.name.resolved.as_ref().unwrap().clone(), var_def);
            }
            Definition::StructDef(struct_def) => {
                definition_table.struct_map.insert(
                    struct_def.name.resolved.as_ref().unwrap().clone(),
                    struct_def,
                );
            }
            Definition::FnDef(fn_def) => {
                definition_table
                    .fn_map
                    .insert(fn_def.name.resolved.as_ref().unwrap().clone(), fn_def);
            }
        }
    }

    return definition_table;
}
