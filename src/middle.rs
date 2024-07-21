use crate::middle::global_definition_table::GlobalDefinitionTable;
use crate::middle::ir_types::IRModule;
use crate::modules::ModuleId;

mod global_definition_table;
pub mod ir_types;

pub fn generate_ir(
    module_id: ModuleId,
    global_definition_table: GlobalDefinitionTable,
) -> IRModule {
    return IRModule {};
}
