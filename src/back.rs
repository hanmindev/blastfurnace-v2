pub mod commands;
pub mod hmasm_types;

use crate::back::hmasm_types::HmasmFile;
use crate::middle::ir_types::IRModule;

pub fn ir_to_asm(file_name: &str, ir: IRModule) -> HmasmFile {
    return HmasmFile {
        file_name: file_name.to_string(),
        functions: vec![hmasm_types::HmasmFunction {
            name: "main".to_string(),
            scope: hmasm_types::HmasmScope {
                instructions: vec![hmasm_types::HmasmInstruction::MCommand(
                    "say Hello, world!".to_string(),
                )],
            },
        }],
    };
}
