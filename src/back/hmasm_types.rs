use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HmasmInstruction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmasmFunction {
    pub name: String,
    pub instructions: Vec<HmasmInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmasmFile {
    pub file_name: String,
    pub functions: Vec<HmasmFunction>,
}
