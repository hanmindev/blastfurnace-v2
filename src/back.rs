pub mod hmasm_types;

use crate::back::hmasm_types::HmasmFile;
use crate::middle::ir_types::IRModule;

pub fn ir_to_asm(file_name: &str, ir: IRModule) -> HmasmFile {
    return HmasmFile {
        file_name: file_name.to_string(),
        functions: vec![],
    };
}
