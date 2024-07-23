use crate::middle::global_definition_table::GlobalDefinitionTable;
use crate::middle::types::{IRFunction, IRInstruction, IRModule};
use crate::modules::ModuleId;

pub mod global_definition_table;
pub mod types;

pub fn generate_ir(
    module_id: &ModuleId,
    global_definition_table: &GlobalDefinitionTable,
) -> IRModule {
    // This is a placeholder implementation that just prints "Hello, world!".
    return IRModule {
        id: module_id.clone(),
        functions: vec![IRFunction {
            name: "main".to_string(),
            instructions: vec![IRInstruction::MCommand("say Hello, world!".to_string())],
        }],
    };
}
