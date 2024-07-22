use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HmasmInstruction {
    MCommand(String),
    Scope(HmasmScope),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmasmScope {
    pub instructions: Vec<HmasmInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmasmFunction {
    pub name: String,
    pub scope: HmasmScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmasmFile {
    pub file_name: String,
    pub functions: Vec<HmasmFunction>,
}
